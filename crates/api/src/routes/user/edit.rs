use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::user::ChangePasswordRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "user", summary = "Modify user", description = r###""###)]
async fn exec(Json(data): Json<ChangePasswordRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/change-password").route(apistos::web::put().to(exec))
}
