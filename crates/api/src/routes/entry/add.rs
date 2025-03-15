use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::entry::AddEntryRequest;

use crate::error::ApiResult;

#[api_operation(
    tag = "entry",
    summary = "Adds a Series/Movie",
    description = r###""###
)]
async fn exec(Json(data): Json<AddEntryRequest>) -> ApiResult<CreatedJson<u8>> {
    Ok(CreatedJson(0))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
