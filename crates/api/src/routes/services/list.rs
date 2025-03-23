use std::sync::Mutex;

use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::Role;

use crate::services::Services;

#[api_operation(
    tag = "services",
    summary = "List available services",
    description = r###""###
)]
async fn exec(services: Data<Mutex<Services>>) -> Json<Vec<String>> {
    Json(services.lock().unwrap().services())
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/list")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
