use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::file::SetEpisodeRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "file",
    summary = "Validate the detected episode",
    description = r###""###
)]
async fn exec(Json(data): Json<SetEpisodeRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/set-episode").route(apistos::web::put().to(exec))
}
