use std::fs::read_dir;

use actix_web::web::Json;
use apistos::api_operation;
use structures::{DirRequest, DirResponse, File};

use crate::error::ApiResult;

#[api_operation(
    tag = "file",
    summary = "Lists every file at that path",
    description = r###""###
)]
async fn exec(Json(data): Json<DirRequest>) -> ApiResult<Json<DirResponse>> {
    let items = read_dir(data.path)
        .map(|v| {
            v.flat_map(|v| v.ok())
                .map(|v| v.path())
                .map(|v| File {
                    dir: v.is_dir(),
                    name: v
                        .file_name()
                        .and_then(|v| v.to_str())
                        .unwrap_or_default()
                        .to_owned(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(Json(DirResponse { items }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/dir").route(apistos::web::post().to(exec))
}
