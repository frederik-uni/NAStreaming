use std::sync::Mutex;

use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::Role;
use structures::services::ServiceStateResponse;

use crate::services::Services;

#[api_operation(
    tag = "services",
    summary = "See states of running services",
    description = r###""###
)]
async fn exec(services: Data<Mutex<Services>>) -> Json<ServiceStateResponse> {
    Json(ServiceStateResponse {
        services: services.lock().unwrap().get_states(),
    })
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/state")
        .route(apistos::web::get().to(exec))
        .guard(AuthorityGuard::any([Role::Admin]))
}
