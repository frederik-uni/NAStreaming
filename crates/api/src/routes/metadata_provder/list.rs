use actix_web::web::Json;
use apistos::api_operation;
use models::files::File;
use structures::{
    link::{FileValidate, FileValidationResponse, Unidified},
    Episode, IdArrayRequest,
};

use crate::error::ApiResult;

#[api_operation(
    tag = "metadata-provider",
    summary = "Lists all available services that provide metadata info",
    description = r###""###
)]
async fn exec(Json(ids): Json<IdArrayRequest>) -> ApiResult<Json<FileValidationResponse>> {
    let mut items = vec![];
    for v in File::get_info(ids.ids).await? {
        items.push(FileValidate {
            group_id: todo!(),
            path: v.path,
            info: Unidified {
                try_group: v.info.try_group.map(|v| v.id().key().to_string()),
                name: v.info.name,
                ep_name: v.info.ep_name,
                sure: v.info.sure,
                season: v.info.season,
                episode: v
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
                                big_decimal.to_string(),
                            )
                        }
                    })
                    .collect(),
                year: v.info.year,
                resolutions: v
                    .info
                    .resolutions
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                three_ds: v.info.three_ds.into_iter().map(|v| v.to_string()).collect(),
                extended: v.info.extended.into_iter().map(|v| v.to_string()).collect(),
                kinds: v.info.kinds.into_iter().map(|v| v.to_string()).collect(),
            },
        })
    }

    Ok(Json(FileValidationResponse { items }))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list").route(apistos::web::post().to(exec))
}
