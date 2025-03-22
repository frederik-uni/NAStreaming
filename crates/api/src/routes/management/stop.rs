use std::{thread::spawn, time::Duration};

use actix_web_grants::AuthorityGuard;
use apistos::{actix::AcceptedJson, api_operation};
use models::user::Role;

#[api_operation(tag = "server-management", summary = "", description = r###""###)]
async fn exec() -> AcceptedJson<u16> {
    spawn(|| {
        std::thread::sleep(Duration::from_secs(1));
        std::process::exit(0);
    });
    AcceptedJson(202)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/stop")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::new([Role::Admin]))
}
