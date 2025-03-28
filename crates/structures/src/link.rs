use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Episode;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct LinkOverviewResponse {
    pub data: HashMap<PathBuf, Vec<String>>,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct FileValidate {
    pub file_id: String,
    pub scan_group_id: String,
    pub path: PathBuf,
    pub info: Unidified,
}

#[derive(Deserialize, Serialize)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct FileValidationResponse {
    pub items: Vec<FileValidate>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct Unidified {
    /// Entry id
    pub try_group: Option<String>,
    pub name: Vec<String>,
    pub ep_name: Vec<String>,
    pub sure: bool,
    pub season: Vec<u64>,
    pub episode: Vec<Episode>,
    pub year: Vec<u16>,
    pub resolutions: Vec<String>,
    pub three_ds: Vec<String>,
    pub extended: Vec<String>,
    pub kinds: Vec<String>,
}
