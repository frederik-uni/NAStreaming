use std::{
    collections::HashMap,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{dev::ServiceRequest, web::Data, Error, HttpMessage as _};
use actix_web_grants::authorities::AttachAuthorities as _;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claim {
    kind: JwtType,
    role: Role,
    exp: u128,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Role {
    Admin,
    User,
    None,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum JwtType {
    Access,
    Refresh,
}

#[derive(Debug)]
pub struct AuthService {
    pub secret: Vec<u8>,
    pub claims: Mutex<HashMap<String, Claim>>,
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret = req
        .app_data::<Data<AuthService>>()
        .expect("AuthService is missing");
    if let Some(token) = credentials {
        let token = token.token().trim();
        if !token.is_empty() {
            match secret.get_claim(token) {
                Ok(v) => {
                    if matches!(v.kind, JwtType::Access) {
                        req.attach(vec![v.role]);
                    }
                    let mut ext = req.extensions_mut();
                    ext.insert(v);
                }
                Err(e) => return Err((e.into(), req)),
            }
        } else {
            let v = Claim {
                kind: JwtType::Access,
                role: Role::None,
                exp: 0,
            };
            req.attach(vec![v.role]);
            let mut ext = req.extensions_mut();
            ext.insert(v);
        }
    } else {
        let v = Claim {
            kind: JwtType::Access,
            role: Role::None,
            exp: 0,
        };
        req.attach(vec![v.role]);
        let mut ext = req.extensions_mut();
        ext.insert(v);
    }
    Ok(req)
}

impl AuthService {
    pub fn new(secret: Vec<u8>) -> Self {
        Self {
            claims: Mutex::new(HashMap::new()),
            secret,
        }
    }

    pub fn hash_password(&self, password: &str) -> ApiResult<String> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_hash(&self, password: String, hash: String) -> bool {
        verify(password, &hash).unwrap_or(false)
    }

    pub fn get_claim(&self, token: &str) -> ApiResult<Claim> {
        if let Some(v) = self.claims.lock().unwrap().get(token) {
            if v.exp < now_ms() {
                self.claims.lock().unwrap().remove(token);
                return Err(ApiError::ExpiredToken);
            }
            return Ok(v.clone());
        }
        let claim = self.decode_claim(token);
        if let Ok(claim) = &claim {
            self.claims
                .lock()
                .unwrap()
                .insert(token.to_string(), claim.clone());
        }
        claim
    }

    fn decode_claim(&self, token: &str) -> ApiResult<Claim> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let token =
            decode::<Claim>(token, &decoding_key, &Validation::new(Algorithm::HS512))?.claims;

        if token.exp < now_ms() {
            Err(ApiError::ExpiredToken)
        } else {
            Ok(token)
        }
    }

    pub fn encode_claim(&self, claim: &Claim) -> ApiResult<String> {
        let header = Header::new(Algorithm::HS512);
        jsonwebtoken::encode(&header, claim, &EncodingKey::from_secret(&self.secret))
            .map_err(ApiError::GenerateJwt)
    }
}
