use crate::error::{ApiError, ApiResult};
use actix_web::web::Json;
use apistos::api_operation;
use structures::{entry::search::SearchResponse, search::SearchRequest};

#[api_operation(
    tag = "entry",
    summary = "Searches the Series/Movies",
    description = r###""###
)]
async fn exec(Json(data): Json<SearchRequest>) -> ApiResult<Json<SearchResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/search").route(apistos::web::post().to(exec))
}
