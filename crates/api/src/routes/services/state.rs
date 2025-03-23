use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::Role;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "services",
    summary = "See states of running services",
    description = r###""###
)]
async fn exec() -> ApiResult<Json<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/state")
        .route(apistos::web::get().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
