use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Kind;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct AddLibRequest {
    pub name: String,
    pub kind: Option<Kind>,
    pub path: PathBuf,
    pub discover_path: Option<PathBuf>,

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

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct ScanGroup {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub detect_path: Option<PathBuf>,
    pub display_order: Vec<String>,
    pub index_order: Vec<String>,
    pub series: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct ScanGroupListResponse {
    pub scan_groups: Vec<ScanGroup>,
}
