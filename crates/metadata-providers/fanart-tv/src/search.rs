use metadata_provider::{
    async_trait,
    fetcher::{Client, Url},
    search::{Capabilities, SearchProvider, SearchResult},
};

use crate::Instance;

#[async_trait]
impl SearchProvider for Instance {
    fn capabilities(&self) -> Vec<Capabilities> {
        vec![Capabilities::Category]
    }

    async fn search(
        &self,
        client: &Client,
        query: &str,
        _year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<metadata_provider::search::SearchResult>, metadata_provider::Error> {
        let mut url = Url::parse(&format!(
            "https://fanart.tv/api/search.php?section={}",
            match series {
                None => "everything",
                Some(false) => "movies",
                Some(true) => "tv",
            }
        ))
        .unwrap();

        url.query_pairs_mut().append_pair("s", query);
        let data: Vec<Root1> = client
            .get(url)
            .with_user_agent()
            .send()
            .await?
            .json()
            .await?;

        Ok(data
            .into_iter()
            .map(|v| SearchResult {
                id: format!("{}-{}", v.r#type, v.id),
                names: vec![v.title],
                overview: None,
                cover: v.poster,
                kind: Some(v.r#type),
                first_aired: None,
            })
            .collect())
    }
}

#[derive(Debug, serde::Deserialize)]
struct Root1 {
    id: String,
    link: String,
    poster: Option<String>,
    title: String,
    r#type: String,
}
