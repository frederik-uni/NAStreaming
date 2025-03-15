use apistos::{actix::CreatedJson, api_operation};

use crate::error::ApiResult;

#[api_operation(tag = "user", summary = "Sign in", description = r###""###)]
async fn exec() -> ApiResult<CreatedJson<u8>> {
    Ok(CreatedJson(0))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/sign-in").route(apistos::web::get().to(exec))
}
