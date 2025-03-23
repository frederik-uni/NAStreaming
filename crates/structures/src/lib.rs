use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

pub mod entry;
pub mod episodes;
pub mod file;
pub mod init;
pub mod metadata_provider;
pub mod movie_lib;
pub mod search;
pub mod services;
pub mod user;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct IdRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub enum Status {
    Announced,
    Completed,
    Continuing,
    Ended,
    Filming,
    PreProduction,
    Released,
    Upcoming,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub enum Kind {
    Movie,
    Series,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct PaginationRequest {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct HomeResponse {
    data: HashMap<String, Vec<MovieItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct MovieItem {
    title: String,
    description: Option<String>,
    img: Option<String>,
    color: String,
    progress: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct DirRequest {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct DirResponse {
    pub items: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "openapi",
    derive(schemars::JsonSchema, apistos::ApiComponent)
)]
pub struct File {
    pub dir: bool,
    pub name: String,
}
