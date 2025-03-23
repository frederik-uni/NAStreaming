use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::{actix::CreatedJson, api_operation};
use models::{scan_groups::ScanGroup, user::Role};
use structures::movie_lib::EditLibRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "lib", summary = "Modifies settings", description = r###""###)]
async fn exec(Json(data): Json<EditLibRequest>) -> ApiResult<CreatedJson<u16>> {
    if let Some(name) = data.name {
        ScanGroup::update_name(&data.id, name).await?;
    }
    if let Some(prefered_display_order) = data.prefered_display_order {
        ScanGroup::update_prefered_display_order(&data.id, prefered_display_order).await?;
    }
    if let Some(prefered_index_order) = data.prefered_index_order {
        ScanGroup::update_prefered_index_order(&data.id, prefered_index_order).await?;
    }

    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/edit")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
