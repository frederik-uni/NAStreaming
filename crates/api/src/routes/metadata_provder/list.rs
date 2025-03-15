use actix_web::web::Json;
use apistos::api_operation;
use structures::{metadata_provider::ListProvidersResponse, PaginationRequest};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "metadata-provider",
    summary = "Lists all available services that provide metadata info",
    description = r###""###
)]
async fn exec(Json(data): Json<PaginationRequest>) -> ApiResult<Json<ListProvidersResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list").route(apistos::web::post().to(exec))
}
