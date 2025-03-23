use std::sync::Mutex;

use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::{actix::AcceptedJson, api_operation};
use models::user::Role;
use structures::services::DispatchRequest;

use crate::{
    error::ApiResult,
    services::{Services, Value},
};

#[api_operation(tag = "services", summary = "Start services", description = r###""###)]
async fn exec(
    Json(data): Json<DispatchRequest>,
    services: Data<Mutex<Services>>,
) -> ApiResult<AcceptedJson<u16>> {
    services.lock().unwrap().start_with_ctx(
        &data.service,
        match data.ctx.map(Value::String) {
            Some(v) => vec![v],
            None => vec![],
        },
    )?;
    Ok(AcceptedJson(202))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/dispatch")
        .route(apistos::web::put().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
