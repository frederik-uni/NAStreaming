use std::sync::Mutex;

use actix_web::web::{Data, Json};
use apistos::{actix::CreatedJson, api_operation};
use structures::{init::InitRequest, user::JWTReponse};

use crate::{
    app_data::UserExists,
    error::{ApiError, ApiResult},
    routes::user,
    services::{auth::AuthService, scan::ScanService},
};

use super::lib;

#[api_operation(
    tag = "init",
    summary = "returns if the service is set up",
    description = r###"If this is the first time the service is started, it will return true. Otherwise, it will return false."###
)]
async fn exec(state: Data<Mutex<UserExists>>) -> Json<bool> {
    Json(!state.lock().unwrap().exists)
}

#[api_operation(
    tag = "init",
    summary = "creates the first user & the first group",
    description = r###""###
)]
async fn exec2(
    Json(body): Json<InitRequest>,
    state: Data<Mutex<UserExists>>,
    scan_service: Data<ScanService>,
    auth_service: Data<AuthService>,
) -> ApiResult<CreatedJson<JWTReponse>> {
    if state.lock().unwrap().exists {
        return Err(ApiError::NoPermission);
    }
    lib::add::exec(Json(body.group), scan_service).await?;
    let user = user::create::exec(Json(body.user), auth_service).await?;
    state.lock().unwrap().exists = true;
    Ok(user)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/init")
        .route(apistos::web::get().to(exec))
        .route(apistos::web::put().to(exec2))
}
