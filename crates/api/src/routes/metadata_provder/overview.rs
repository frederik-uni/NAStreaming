use actix_web::web::Json;
use apistos::api_operation;
use models::files::File;
use structures::link::LinkOverviewResponse;

use crate::error::ApiResult;

#[api_operation(
    tag = "metadata-provider",
    summary = "Groups by path and returns a map of file ids",
    description = r###""###
)]
async fn exec() -> ApiResult<Json<LinkOverviewResponse>> {
    Ok(Json(LinkOverviewResponse {
        data: File::group()
            .await?
            .into_iter()
            .map(|v| {
                (
                    v.0,
                    v.1.into_iter()
                        .map(|v| v.key().to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .collect(),
    }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/overview").route(apistos::web::get().to(exec))
}
