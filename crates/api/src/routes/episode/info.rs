use actix_web::web::Json;
use apistos::api_operation;
use structures::{episodes::EpisodeResponse, IdRequest};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "episode",
    summary = "Retrieves info about the episode",
    description = r###""###
)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<Json<EpisodeResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/info").route(apistos::web::post().to(exec))
}
