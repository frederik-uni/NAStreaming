use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::{actix::CreatedJson, api_operation};
use structures::user::{JWTReponse, NewUserRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::Role,
};

#[api_operation(tag = "user", summary = "Create a user", description = r###""###)]
pub async fn exec(Json(data): Json<NewUserRequest>) -> ApiResult<CreatedJson<JWTReponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/create")
        .route(apistos::web::put().to(exec))
        .guard(AuthorityGuard::new([Role::Admin]))
}
