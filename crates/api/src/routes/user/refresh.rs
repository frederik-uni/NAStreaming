use actix_web::web::{Data, Json};
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use models::{
    user::{Role, User},
    DbUtils,
};
use structures::user::{JWTReponse, RefreshRequest};

use crate::{
    error::{ApiError, ApiResult},
    services::auth::AuthService,
};

#[api_operation(tag = "user", summary = "Sign in", description = r###""###)]
async fn exec(
    Json(data): Json<RefreshRequest>,
    auth_service: Data<AuthService>,
) -> ApiResult<Json<JWTReponse>> {
    let claim = auth_service.get_claim(&data.refresh_token)?;
    let user = User::get(&claim.user_id)
        .await?
        .ok_or(ApiError::NotFoundInDb)?;
    auth_service.new_jwt_response(&user).map(Json)
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/refresh")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::User, Role::Admin]))
}
