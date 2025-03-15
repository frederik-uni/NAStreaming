use serde::{Deserialize, Serialize};

pub mod entry;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct IdRequest {
    pub id: String,
}
