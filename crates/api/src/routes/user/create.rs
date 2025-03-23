use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::{actix::CreatedJson, api_operation};
use chrono::{DateTime, Utc};
use models::{
    user::{Role, User},
    DbUtils as _,
};
use structures::user::{JWTReponse, NewUserRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::AuthService,
};

#[api_operation(tag = "user", summary = "Create a user", description = r###""###)]
pub async fn exec(
    Json(data): Json<NewUserRequest>,
    auth_service: Data<AuthService>,
) -> ApiResult<CreatedJson<JWTReponse>> {
    if let Some(email) = &data.email {
        if User::has_email(email).await? {
            return Err(ApiError::Conflict("email".to_string()));
        }
    }

    if User::has_name(&data.name).await? {
        return Err(ApiError::Conflict("name".to_string()));
    }

    let user = User {
        name: data.name,
        email: data.email,
        role: match data.admin {
            true => Role::Admin,
            false => Role::User,
        },
        password_hash: auth_service.hash_password(&data.password)?,
        updated: Default::default(),
        created: Default::default(),
        birthdate: DateTime::parse_from_rfc3339(&data.birthdate)?
            .with_timezone(&Utc)
            .into(),
        icon: None,
    }
    .add()
    .await?;
    auth_service.new_jwt_response(&user).map(CreatedJson)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/create")
        .route(apistos::web::put().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
