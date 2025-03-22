use actix_web::web::{Data, Json};
use apistos::{actix::CreatedJson, api_operation};
use models::{scan_groups::ScanGroup, DbUtils as _};
use structures::{movie_lib::AddLibRequest, Kind};

use crate::{error::ApiResult, services::scan::ScanService};

#[api_operation(
    tag = "lib",
    summary = "Creates a new indexing lib. This is a group, which sets rules like indexing path & indexing rules",
    description = r###""###
)]
pub async fn exec(
    Json(data): Json<AddLibRequest>,
    scan_service: Data<ScanService>,
) -> ApiResult<CreatedJson<u16>> {
    let group = ScanGroup {
        name: data.name,
        path: data.path,
        detect_path: data.discover_path,
        display_order: data.prefered_display_order,
        index_order: data.prefered_index_order,
        series: data.kind.map(|v| Kind::Series == v),
    }
    .add()
    .await?;
    scan_service.scan(group);
    Ok(CreatedJson(202))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
