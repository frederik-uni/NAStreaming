use actix_web::web::Json;
use actix_web_grants::AuthorityGuard;
use apistos::api_operation;
use chrono::{DateTime, Utc};
use models::{
    user::{Role, User},
    DbUtils as _,
};
use structures::{user::UserInfoResponse, IdRequest};

use crate::error::{ApiError, ApiResult};

#[api_operation(tag = "user", summary = "User info", description = r###""###)]
async fn exec(Json(id): Json<IdRequest>) -> ApiResult<Json<UserInfoResponse>> {
    let user = User::get(&id.id).await?.ok_or(ApiError::NotFoundInDb)?;
    Ok(Json(UserInfoResponse {
        id: user.id.key().to_string(),
        user: user.data.name,
        email: user.data.email,
        icon: user.data.icon,
        age: calculate_age(user.data.birthdate.into_inner().0) as u32,
    }))
}

fn calculate_age(birth_date: DateTime<Utc>) -> i64 {
    let now = Utc::now();
    let duration = now.signed_duration_since(birth_date);
    let years = duration.num_days() / 365; // Approximate age in years
    years
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/info")
        .route(apistos::web::post().to(exec))
        .guard(AuthorityGuard::any([Role::Admin, Role::User]))
}
