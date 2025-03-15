use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use structures::user::NewUserRequest;

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "user", summary = "Create a user", description = r###""###)]
async fn exec(Json(data): Json<NewUserRequest>) -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/create").route(apistos::web::put().to(exec))
}
