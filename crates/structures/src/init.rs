use serde::{Deserialize, Serialize};

use crate::{movie_lib::AddLibRequest, user::NewUserRequest};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct InitRequest {
    pub user: NewUserRequest,
    pub group: AddLibRequest,
}
