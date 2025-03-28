use std::fmt::Display;

use async_trait::async_trait;

use crate::{fetcher::Client, Error};

pub enum Capabilities {
    Category,
    Year,
    TitleExact,
}

impl Display for Capabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Capabilities::Category => "Category",
                Capabilities::Year => "Year",
                Capabilities::TitleExact => "TitleExact",
            }
        )
    }
}
#[async_trait]
pub trait SearchProvider {
    fn capabilities(&self) -> Vec<Capabilities>;
    async fn search(
        &self,
        client: &Client,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<SearchResult>, Error>;
}

#[derive(Debug)]
pub struct SearchResult {
    pub id: String,
    pub names: Vec<String>,
    pub overview: Option<String>,
    pub cover: Option<String>,
    pub kind: Option<String>,
    pub first_aired: Option<String>,
}
