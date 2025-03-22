use crate::error::{ApiError, ApiResult};
use actix_web::web::Json;
use apistos::{actix::AcceptedJson, api_operation};
use structures::IdRequest;

#[api_operation(
    tag = "entry",
    summary = "Updates the metadata of an entry",
    description = r###""###
)]
async fn exec(Json(data): Json<IdRequest>) -> ApiResult<AcceptedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/update-metadata").route(apistos::web::post().to(exec))
}
