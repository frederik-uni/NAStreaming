use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::movie_lib::AddLibRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "lib",
    summary = "Creates a new indexing lib. This is a group, which sets rules like indexing path & indexing rules",
    description = r###""###
)]
async fn exec(Json(data): Json<AddLibRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
