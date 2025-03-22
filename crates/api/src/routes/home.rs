use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::Role;
use structures::HomeResponse;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "home",
    summary = "Homeapge data",
    description = r###"Homeapge data"###
)]
async fn exec() -> ApiResult<Json<HomeResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/home")
        .route(apistos::web::get().to(exec))
        .guard(AuthorityGuard::new([Role::Admin, Role::User]))
}
