use actix_web_grants::AuthorityGuard;
use apistos::{actix::AcceptedJson, api_operation};
use models::user::Role;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "services", summary = "Start services", description = r###""###)]
async fn exec() -> ApiResult<AcceptedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/dispatch")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
