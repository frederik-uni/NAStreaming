use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::file::SetEntryRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "file",
    summary = "Links file to Entry(Movie/Series)",
    description = r###""###
)]
async fn exec(Json(data): Json<SetEntryRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/set-entry").route(apistos::web::put().to(exec))
}
