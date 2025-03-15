use apistos::{actix::CreatedJson, api_operation};

use crate::error::{ApiError, ApiResult};

#[api_operation(
    tag = "entry",
    summary = "Edits the Series/Movie",
    description = r###""###
)]
async fn exec() -> ApiResult<CreatedJson<u16>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/edit").route(apistos::web::put().to(exec))
}
