use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::metadata_provider::LinkEntryRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "metadata-provider",
    summary = "Links extern website to Series/Movie",
    description = r###""###
)]
async fn exec(Json(data): Json<LinkEntryRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/link-entry").route(apistos::web::put().to(exec))
}
