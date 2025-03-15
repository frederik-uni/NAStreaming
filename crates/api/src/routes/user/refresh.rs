use actix_web::web::Json;
use apistos::api_operation;
use structures::user::{JWTReponse, RefreshRequest};

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "user", summary = "Sign in", description = r###""###)]
async fn exec(Json(data): Json<RefreshRequest>) -> ApiResult<Json<JWTReponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/refresh").route(apistos::web::post().to(exec))
}
