use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::{user::Role, DbUtils as _};
use structures::{movie_lib::ScanGroupListResponse, PaginationRequest};

use crate::error::ApiResult;

#[api_operation(
    tag = "lib",
    summary = "Lists every indexing group",
    description = r###""###
)]
async fn exec(Json(_data): Json<PaginationRequest>) -> ApiResult<Json<ScanGroupListResponse>> {
    let all = models::scan_groups::ScanGroup::all().await?;
    Ok(Json(ScanGroupListResponse {
        scan_groups: all
            .into_iter()
            .map(|v| structures::movie_lib::ScanGroup {
                id: v.id.key().to_string(),
                name: v.data.name,
                path: v.data.path,
                detect_path: v.data.detect_path,
                display_order: v.data.display_order,
                index_order: v.data.index_order,
                series: v.data.series,
            })
            .collect(),
    }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
