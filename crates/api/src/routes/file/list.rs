use actix_web::web::Json;
use apistos::api_operation;
use structures::{file::ListFiles, PaginationRequest};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "file",
    summary = "Lists every file that has no entry",
    description = r###""###
)]
async fn exec(Json(data): Json<PaginationRequest>) -> ApiResult<Json<ListFiles>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list").route(apistos::web::post().to(exec))
}
