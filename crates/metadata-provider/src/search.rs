use crate::Error;

pub enum Capabilities {
    Category,
    Year,
    TitleExact,
}
pub trait SearchProvider {
    fn capabilities(&self) -> Vec<Capabilities>;
    fn search(
        &self,
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
