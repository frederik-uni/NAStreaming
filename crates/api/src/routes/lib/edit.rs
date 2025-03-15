use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::movie_lib::EditLibRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "lib", summary = "Modifies settings", description = r###""###)]
async fn exec(Json(id): Json<EditLibRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/edit").route(apistos::web::post().to(exec))
}
