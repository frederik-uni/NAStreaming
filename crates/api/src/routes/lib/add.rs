use std::{fs::create_dir_all, sync::Mutex};

use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::{actix::CreatedJson, api_operation};
use models::{scan_groups::ScanGroup, user::Role, DbUtils as _};
use structures::{movie_lib::AddLibRequest, Kind};

use crate::{
    error::ApiResult,
    services::{Services, Value},
};

#[api_operation(
    tag = "lib",
    summary = "Creates a new indexing lib. This is a group, which sets rules like indexing path & indexing rules",
    description = r###""###
)]
pub async fn exec(
    Json(data): Json<AddLibRequest>,
    services: Data<Mutex<Services>>,
) -> ApiResult<CreatedJson<u16>> {
    let _ = create_dir_all(&data.path);
    if let Some(path) = &data.discover_path {
        let _ = create_dir_all(path);
    }
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

    services.lock().unwrap().start_with_ctx(
        "scan",
        vec![Value::Bool(true), Value::String(group.id.key().to_string())],
    )?;
    services.lock().unwrap().start_with_ctx(
        "scan",
        vec![
            Value::Bool(false),
            Value::String(group.id.key().to_string()),
        ],
    )?;
    Ok(CreatedJson(202))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add")
        .route(apistos::web::put().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
