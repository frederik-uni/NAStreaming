use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::episodes::AddEpisodeRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "episode", summary = "Add a episode", description = r###""###)]
async fn exec(Json(data): Json<AddEpisodeRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
