use metadata_provider::{
    fetcher::{reqwest::header::AUTHORIZATION, Url},
    search::{Capabilities, SearchProvider, SearchResult},
};
use serde::Deserialize;

use crate::Instance;

impl SearchProvider for Instance {
    fn capabilities(&self) -> Vec<Capabilities> {
        vec![
            Capabilities::TitleExact,
            Capabilities::Year,
            Capabilities::Category,
        ]
    }

    fn search(
        &self,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<metadata_provider::search::SearchResult>, metadata_provider::Error> {
        let kind = match series {
            None => "multi",
            Some(true) => "tv",
            Some(false) => "movie",
        };
        let mut url = Url::parse(&format!("https://api.themoviedb.org/3/search/{kind}",)).unwrap();
        url.query_pairs_mut()
            .append_pair("query", query)
            .append_pair("include_adult", "true")
            .append_pair("page", "1");
        if let Some(year) = year {
            url.query_pairs_mut().append_pair("year", &year.to_string());
        }

        let data: Root1 = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()?
            .json()?;
        Ok(data
            .results
            .into_iter()
            .map(|v| SearchResult {
                id: format!(
                    "{}-{}",
                    v.media_type.as_ref().unwrap_or(&kind.to_owned()),
                    v.id
                ),
                names: match v.original_title {
                    Some(original_title) => vec![v.title, original_title],
                    None => vec![v.title],
                },
                overview: Some(v.overview),
                cover: v
                    .poster_path
                    .map(|v| format!("https://media.themoviedb.org/t/p/w440_and_h660_face{v}")),
                kind: Some(v.media_type.unwrap_or(kind.to_owned())),
                first_aired: v.release_date,
            })
            .collect())
    }
}

#[derive(Deserialize)]
struct Results1 {
    id: i64,
    media_type: Option<String>,
    poster_path: Option<String>,
    #[serde(alias = "name")]
    title: String,
    #[serde(alias = "original_name")]
    original_title: Option<String>,
    overview: String,
    #[serde(alias = "first_air_date")]
    release_date: Option<String>,
}

#[derive(Deserialize)]
struct Root1 {
    results: Vec<Results1>,
}
