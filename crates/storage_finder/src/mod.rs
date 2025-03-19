pub mod search;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct AddEntryRequest {
    pub name: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct EntryInfoResponse {
    titles: HashMap<String, Vec<String>>,
}
