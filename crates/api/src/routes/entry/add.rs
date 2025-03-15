use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::entry::AddEntryRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "entry",
    summary = "Adds a Series/Movie",
    description = r###""###
)]
async fn exec(Json(data): Json<AddEntryRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
