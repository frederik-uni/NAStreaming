pub mod fetcher;
pub mod search;

use search::SearchProvider;

pub use async_trait::async_trait;

pub enum State {
    Planed,
    WIP,
    Running,
}
pub enum DataRetrievel {
    Unknown,
    Scraping,
    Api,
    SearchInternalApiInfoApi,
    SearchScraperInfoApi,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(::reqwest::Error),
    Serde(serde_json::Error),
    NodeNotFound,
    InvalidId,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(value: ::reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

pub enum Issue {
    SeasonsAreSeperateEntries,
}
pub trait MetadataProvider {
    /// Display string
    fn name(&self) -> &'static str;
    /// State of development
    fn state(&self) -> State;

    fn id_to_url(&self, id: &str) -> String;

    fn data_retrievel(&self) -> DataRetrievel {
        DataRetrievel::Unknown
    }

    fn issues(&self) -> Vec<Issue> {
        vec![]
    }
    /// Original site
    fn origin(&self) -> &'static str;
    /// Returns search instance
    fn search(&self) -> Option<&dyn SearchProvider>;
    /// Returns info instance
    fn info(&self) -> Option<Box<dyn InfoProvider>>;
}

pub trait InfoProvider {}
