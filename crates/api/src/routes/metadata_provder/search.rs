use actix_web::web::{Data, Json};
use apistos::api_operation;
use structures::metadata_provider::{
    InfoItem, MetadataProviderSearch, MetadataProviderSearchResponse,
};

use crate::{error::ApiResult, services::metadata::MetadataService};
#[api_operation(
    tag = "metadata-provider",
    summary = "Search metadata provider",
    description = r###""###
)]
async fn exec(
    Json(data): Json<MetadataProviderSearch>,
    metadata_service: Data<MetadataService>,
) -> ApiResult<Json<MetadataProviderSearchResponse>> {
    let items = metadata_service
        .search(&data.id, &data.query, data.year, data.series)
        .await?
        .into_iter()
        .map(|v| InfoItem {
            id: v.id,
            names: v.names,
            overview: v.overview,
            cover: v.cover,
            kind: v.kind,
            first_aired: v.first_aired,
        })
        .collect();
    Ok(Json(MetadataProviderSearchResponse { items }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/search").route(apistos::web::post().to(exec))
}
