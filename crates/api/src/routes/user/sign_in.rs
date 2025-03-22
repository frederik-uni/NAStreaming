use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use structures::user::{JWTReponse, SignInRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::Role,
};

#[api_operation(tag = "user", summary = "Sign in", description = r###""###)]
async fn exec(Json(data): Json<SignInRequest>) -> ApiResult<Json<JWTReponse>> {
    Err(ApiError::NotImplemented)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/sign-in")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::new([Role::None]))
}
