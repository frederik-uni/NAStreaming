use actix_web::web::Json;
use apistos::api_operation;
use structures::IdRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "episode",
    summary = "Deletes everything including the files",
    description = r###""###
)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<Json<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/delete-full").route(apistos::web::delete().to(exec))
}
