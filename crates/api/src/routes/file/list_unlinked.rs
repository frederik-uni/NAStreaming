use std::{collections::HashMap, path::PathBuf};

use actix_web::web::Json;
use apistos::api_operation;
use models::{files::File, scan_groups::ScanGroup};
use structures::{
    link::{FileValidate, FileValidationResponse, Unidified},
    Episode, IdArrayRequest,
};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "file",
    summary = "Lists every file that has no entry",
    description = r###""###
)]
async fn exec(Json(ids): Json<IdArrayRequest>) -> ApiResult<Json<FileValidationResponse>> {
    let mut items = vec![];
    let mut cache: HashMap<PathBuf, String> = HashMap::new();
    let mut cache2: HashMap<PathBuf, Option<String>> = HashMap::new();

    for v in File::get_info(ids.ids).await? {
        let scan_group_id = match cache.get(&v.data.root_path) {
            Some(v) => Some(v.clone()),
            None => {
                let found = ScanGroup::find_by_path(&v.data.root_path).await?;
                let first = found.get(0).map(|v| v.1.key().to_string());
                for item in found {
                    cache.insert(item.0, item.1.key().to_string());
                }
                first
            }
        }
        .ok_or(ApiError::NotFoundInDb)?;
        let mut group_path = v.data.root_path.clone();
        if let Some(v) = v.data.path.components().next() {
            group_path = group_path.join(v);
        }
        let try_group = match cache2.get(&group_path) {
            Some(v) => v.clone(),
            None => {
                let related = File::find_related(group_path.clone())
                    .await?
                    .map(|v| v.id().key().to_string());
                cache2.insert(group_path, related.clone());
                related
            }
        };
        items.push(FileValidate {
            file_id: v.id.key().to_string(),
            scan_group_id,
            path: v.data.path,
            info: Unidified {
                try_group,
                name: v.data.info.name,
                ep_name: v.data.info.ep_name,
                sure: v.data.info.sure,
                season: v.data.info.season,
                episode: v
                    .data
                    .info
                    .episode
                    .into_iter()
                    .map(|v| match v {
                        storage_finder::Episode::List(big_decimals) => {
                            Episode::List(big_decimals.into_iter().map(|v| v.to_string()).collect())
                        }
                        storage_finder::Episode::Single(big_decimal) => {
                            Episode::Single(big_decimal.to_string())
                        }
                        storage_finder::Episode::Part(big_decimal, p) => {
                            Episode::Part(big_decimal.to_string(), p)
                        }
                        storage_finder::Episode::RangeInclusive(big_decimal, big_decimal1) => {
                            Episode::RangeInclusive(
                                big_decimal.to_string(),
                                big_decimal1.to_string(),
                            )
                        }
                    })
                    .collect(),
                year: v.data.info.year,
                resolutions: v
                    .data
                    .info
                    .resolutions
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                three_ds: v
                    .data
                    .info
                    .three_ds
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                extended: v
                    .data
                    .info
                    .extended
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                kinds: v
                    .data
                    .info
                    .kinds
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
            },
        })
    }

    Ok(Json(FileValidationResponse { items }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list-unlinked").route(apistos::web::post().to(exec))
}
