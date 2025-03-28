use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct LinkEntryRequest {
    pub entry_id: String,
    pub metadata_provider: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct MetadataProviderSearch {
    pub id: String,
    pub query: String,
    pub year: Option<u16>,
    pub series: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct MetadataProviderSearchResponse {
    pub items: Vec<InfoItem>,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct ProviderResponse {
    pub list: Vec<Provider>,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct Provider {
    pub id: String,
    pub name: &'static str,
    pub origin: &'static str,
    pub state: String,
    pub data_retrievel: String,
    pub search: Option<Vec<String>>,
    pub info: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct InfoItem {
    pub id: String,
    pub names: Vec<String>,
    pub overview: Option<String>,
    pub cover: Option<String>,
    pub kind: Option<String>,
    pub first_aired: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct ListProvidersResponse {
    pub items: Vec<String>,
}
