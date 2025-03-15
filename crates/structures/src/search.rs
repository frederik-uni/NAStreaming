use crate::Kind;
use crate::Status;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct SearchRequest {
    pub order: Order,
    pub query: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub status: Status,
    pub kind: Kind,
    pub network: Vec<String>,
    pub year: Option<u16>,
    pub order_desc: bool,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub enum Order {
    Alphabetical,
    Created,
    Updated,
    LatestRelease,
}
