use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use structures::{user::UserInfoResponse, IdRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::Role,
};

#[api_operation(tag = "user", summary = "User info", description = r###""###)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<Json<UserInfoResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/info")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::new([Role::Admin, Role::User]))
}
