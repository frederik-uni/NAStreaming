use std::{
    process::Command,
    thread::{self, spawn},
    time::Duration,
};

use actix_web_grants::AuthorityGuard;
use apistos::{actix::AcceptedJson, api_operation};

use crate::services::auth::Role;

#[api_operation(tag = "server-management", summary = "", description = r###""###)]
async fn exec() -> AcceptedJson<u16> {
    if let (Ok(currend_dir), Ok(current_exe)) = (std::env::current_dir(), std::env::current_exe()) {
        spawn(|| {
            std::thread::sleep(Duration::from_secs(1));
            let _ = Command::new(current_exe).current_dir(currend_dir).spawn();
            std::process::exit(0);
        });
        AcceptedJson(202)
    } else {
        AcceptedJson(500)
    }
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/restart")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::new([Role::Admin]))
}
