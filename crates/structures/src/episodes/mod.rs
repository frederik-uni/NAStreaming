use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct AddEpisodeRequest {
    pub name: HashMap<String, String>,
    pub season: i64,
    ///float
    pub episode: Option<String>,
    pub description: Option<String>,
    pub first_aired: Option<u64>,
    pub production_company: Option<String>,
    pub network: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct EpisodeResponse {
    pub names: HashMap<String, String>,
    pub description: HashMap<String, String>,
    ///ms
    pub duration: u64,
    pub progress: Option<u64>,
    pub season: i64,
    pub episode: Option<String>,
    pub network: Option<String>,
    pub first_aired: Option<u64>,
    pub production_company: Option<String>,
}
