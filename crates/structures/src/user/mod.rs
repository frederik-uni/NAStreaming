use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct NewUserRequest {
    pub name: String,
    pub email: Option<String>,
    pub password: String,
    /// rfc3339
    pub birthdate: String,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct ChangePasswordRequest {
    pub reset_code: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct JWTReponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct UserInfoResponse {
    pub id: String,
    pub user: String,
    pub email: Option<String>,
    pub icon: Option<String>,
    pub age: u32,
}
