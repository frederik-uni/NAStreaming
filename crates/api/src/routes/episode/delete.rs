use apistos::{actix::CreatedJson, api_operation};

use crate::error::ApiResult;

#[api_operation(
    tag = "episode",
    summary = "Deletes only the db entry",
    description = r###""###
)]
async fn exec() -> ApiResult<CreatedJson<u8>> {
    Ok(CreatedJson(0))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/delete").route(apistos::web::delete().to(exec))
}
