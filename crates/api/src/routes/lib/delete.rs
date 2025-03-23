use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::Role;
use structures::IdRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "lib",
    summary = "Deletes only the db lib",
    description = r###""###
)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<Json<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/delete")
        .route(apistos::web::delete().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
