use std::collections::HashMap;

use metadata_provider::search::{Capabilities, SearchProvider, SearchResult};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

use crate::Instance;

impl SearchProvider for Instance {
    fn search(
        &self,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<SearchResult>, metadata_provider::Error> {
        let mut url = self.server.join("/v4/search").unwrap();
        url.query_pairs_mut().append_pair("query", query);
        url.query_pairs_mut().append_pair("limit", "250");
        url.query_pairs_mut().append_pair("offset", "0");

        if let Some(series) = series {
            url.query_pairs_mut().append_pair(
                "type",
                match series {
                    true => "series",
                    false => "movie",
                },
            );
        }

        if let Some(y) = year {
            url.query_pairs_mut().append_pair("year", &y.to_string());
        }

        let items: Root1 = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.get_token()?))
            .send()?
            .json()?;
        Ok(items
            .data
            .into_iter()
            .map(|v| {
                let (kind, id) = v.id.split_once("-").unwrap_or_default();
                let title = v.translations.unwrap_or_default().get("eng").cloned();
                let mut names = match title {
                    Some(v) => vec![v],
                    None => vec![],
                };
                names.push(v.name);
                if let Some(name_translated) = v.name_translated {
                    names.push(name_translated);
                }
                if let Some(title) = v.title {
                    names.push(title);
                }
                names.append(&mut v.aliases.unwrap_or_default());
                SearchResult {
                    id: id.to_owned(),
                    names,
                    overview: v
                        .overviews
                        .unwrap_or_default()
                        .get("eng")
                        .cloned()
                        .or(v.overview),
                    cover: Some(v.thumbnail.unwrap_or(v.image_url)),
                    kind: Some(kind.to_owned()),
                    first_aired: v.year,
                }
            })
            .collect())
    }

    fn capabilities(&self) -> Vec<Capabilities> {
        vec![
            Capabilities::TitleExact,
            Capabilities::Year,
            Capabilities::Category,
        ]
    }
}

#[derive(Deserialize, Debug)]
pub struct Root1 {
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub name_translated: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub title: Option<String>,
    pub translations: Option<HashMap<String, String>>,
    pub year: Option<String>,
    pub remote_ids: Option<Vec<RemoteId>>,
    pub overview: Option<String>,
    pub overviews: Option<HashMap<String, String>>,
    pub image_url: String,
    pub thumbnail: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RemoteId {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "sourceName")]
    pub source_name: String,
}
