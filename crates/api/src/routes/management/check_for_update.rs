use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use structures::user::UserInfoResponse;

use crate::{
    error::{ApiError, ApiResult},
    services::auth::Role,
};

#[api_operation(tag = "server-management", summary = "", description = r###""###)]
async fn exec() -> ApiResult<Json<UserInfoResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/needs-update")
        .route(apistos::web::get().to(exec))
        .guard(AuthorityGuard::new([Role::Admin]))
}
