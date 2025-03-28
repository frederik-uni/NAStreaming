use serde::{Deserialize, Serialize};

use crate::Episode;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct Entry {
    pub file_id: String,
    pub season: Option<u32>,
    pub episode: Option<Episode>,
    pub name: Option<String>,
    pub resolution: Option<String>,
    pub kind: Option<String>,
    pub extended: Option<String>,
    pub three_d: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct SetEntryRequest {
    pub entry_id: String,
    pub items: Vec<Entry>,
}
