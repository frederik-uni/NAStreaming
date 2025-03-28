use actix_web::web::{Data, Json};
use apistos::api_operation;
use structures::metadata_provider::ProviderResponse;

use crate::services::metadata::MetadataService;
#[api_operation(
    tag = "metadata-provider",
    summary = "Search metadata provider",
    description = r###""###
)]
async fn exec(metadata_service: Data<MetadataService>) -> Json<ProviderResponse> {
    Json(ProviderResponse {
        list: metadata_service.providers(),
    })
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/providers").route(apistos::web::get().to(exec))
}
