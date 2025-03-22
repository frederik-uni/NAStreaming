use std::sync::Mutex;

use actix_web::web::{Data, Json};
use apistos::{actix::CreatedJson, api_operation};
use structures::{init::InitRequest, user::JWTReponse};

use crate::{app_data::UserExists, error::ApiResult, routes::user};

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
) -> ApiResult<CreatedJson<JWTReponse>> {
    lib::add::exec(Json(body.group)).await?;
    let user = user::create::exec(Json(body.user)).await?;
    state.lock().unwrap().exists = true;
    Ok(user)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/init")
        .route(apistos::web::get().to(exec))
        .route(apistos::web::put().to(exec2))
}
