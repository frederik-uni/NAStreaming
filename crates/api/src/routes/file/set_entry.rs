use apistos::{actix::CreatedJson, api_operation};

use crate::error::ApiResult;

#[api_operation(
    tag = "file",
    summary = "Links file to Entry(Movie/Series)",
    description = r###""###
)]
async fn exec() -> ApiResult<CreatedJson<u8>> {
    Ok(CreatedJson(0))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/set-entry").route(apistos::web::put().to(exec))
}
