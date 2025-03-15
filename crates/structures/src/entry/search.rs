use serde::{Deserialize, Serialize};

use crate::MovieItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct SearchResponse {
    pub items: Vec<MovieItem>,
}
