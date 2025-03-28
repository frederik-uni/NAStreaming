pub mod search;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct AddEntryRequest {
    pub scan_group_id: String,
    /// either url or {provider}/{id}
    ///
    /// tv-db/series-1234
    pub ids: Vec<String>,
    pub series: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct EntryInfoResponse {
    titles: HashMap<String, Vec<String>>,
}
