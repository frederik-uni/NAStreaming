use actix_web::web::Json;
use apistos::api_operation;
use structures::metadata_provider::{MetadataProviderSearch, MetadataProviderSearchResponse};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "metadata-provider",
    summary = "Search metadata provider",
    description = r###""###
)]
async fn exec(
    Json(data): Json<MetadataProviderSearch>,
) -> ApiResult<Json<MetadataProviderSearchResponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/search").route(apistos::web::post().to(exec))
}
