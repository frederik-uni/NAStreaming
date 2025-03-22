use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::user::{Role, User};
use structures::user::{JWTReponse, SignInRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::AuthService,
};

#[api_operation(tag = "user", summary = "Sign in", description = r###""###)]
async fn exec(
    Json(data): Json<SignInRequest>,
    auth_service: Data<AuthService>,
) -> ApiResult<Json<JWTReponse>> {
    let user = User::find(&data.username).await?;
    if auth_service.verify_hash(data.password, &user.data.password_hash) {
        auth_service.new_jwt_response(&user).map(Json)
    } else {
        Err(ApiError::LoginFailed)
    }
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/sign-in")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::new([Role::None, Role::Admin]))
}
