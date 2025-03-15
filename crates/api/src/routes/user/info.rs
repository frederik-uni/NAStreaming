use apistos::{actix::CreatedJson, api_operation};

use crate::error::ApiResult;

#[api_operation(tag = "user", summary = "User info", description = r###""###)]
async fn exec() -> ApiResult<CreatedJson<u8>> {
    Ok(CreatedJson(0))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/info").route(apistos::web::get().to(exec))
}
