pub mod fetcher;
pub mod search;

use std::collections::HashMap;

use search::SearchProvider;

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
    NodeNotFound,
}

impl From<::reqwest::Error> for Error {
    fn from(value: ::reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

pub trait MetadataProvider {
    /// Init instance
    fn new(data: HashMap<String, String>) -> Result<Box<Self>, String>;
    /// Unique id
    fn id() -> &'static str;
    /// Display string
    fn name(&self) -> &'static str;
    /// State of development
    fn state(&self) -> State;

    fn id_to_url(&self, id: &str) -> String;

    fn data_retrievel(&self) -> DataRetrievel {
        DataRetrievel::Unknown
    }
    /// Original site
    fn origin(&self) -> &'static str;
    /// Returns search instance
    fn search(&self) -> Option<&dyn SearchProvider>;
    /// Returns info instance
    fn info(&self) -> Option<Box<dyn InfoProvider>>;
}

pub trait InfoProvider {}
