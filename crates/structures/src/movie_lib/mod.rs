use serde::{Deserialize, Serialize};

use crate::Kind;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct AddLibRequest {
    pub name: String,
    pub kind: Kind,
    pub path: String,
    pub prefered_display_order: Vec<String>,
    pub prefered_index_order: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct EditLibRequest {
    pub id: String,
    pub name: Option<String>,
    pub prefered_display_order: Option<Vec<String>>,
    pub prefered_index_order: Option<Vec<String>>,
}
