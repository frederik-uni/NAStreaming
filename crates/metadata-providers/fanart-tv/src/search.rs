use metadata_provider::{
    fetcher::Url,
    search::{Capabilities, SearchProvider, SearchResult},
};

use crate::Instance;

impl SearchProvider for Instance {
    fn capabilities(&self) -> Vec<Capabilities> {
        vec![Capabilities::Category]
    }

    fn search(
        &self,
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
        let data: Vec<Root1> = self.client.get(url).with_user_agent().send()?.json()?;

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
