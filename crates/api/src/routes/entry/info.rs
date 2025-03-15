use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::{entry::EntryInfoResponse, IdRequest};

use crate::error::{ApiError, ApiResult};
#[api_operation(tag = "entry", summary = "Shows info", description = r###""###)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<CreatedJson<EntryInfoResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/info").route(apistos::web::post().to(exec))
}
