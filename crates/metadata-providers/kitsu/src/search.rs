use std::collections::HashMap;

use metadata_provider::{
    fetcher::Url,
    search::{SearchProvider, SearchResult},
};
use serde::{Deserialize, Serialize};

use crate::Instance;

impl SearchProvider for Instance {
    fn capabilities(&self) -> Vec<metadata_provider::search::Capabilities> {
        todo!()
    }

    fn search(
        &self,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<metadata_provider::search::SearchResult>, metadata_provider::Error> {
        let mut url = Url::parse("https://kitsu.app/api/edge/anime").unwrap();
        url.query_pairs_mut()
            .append_pair("filter[text]", query)
            .append_pair("page[offset]", "0")
            .append_pair("page[limit]", "20")
            .append_pair(
                "fields[anime]",
                "subtype,canonicalTitle,titles,posterImage,description,startDate",
            );
        if let Some(year) = year {
            url.query_pairs_mut()
                .append_pair("filter[year]", &format!("{}..{}", year, year));
        }

        if let Some(series) = series {
            url.query_pairs_mut().append_pair(
                "filter[subtype]",
                match series {
                    true => "tv,ova,ona,specia",
                    false => "movie",
                },
            );
        }
        let data: Root1 = self.client.get(url).send()?.json()?;
        Ok(data
            .data
            .into_iter()
            .map(|v| SearchResult {
                id: v.id,
                names: vec![v.attributes.canonical_title],
                overview: v.attributes.description,
                cover: v.attributes.poster_image.small,
                kind: Some(v.attributes.subtype),
                first_aired: v.attributes.start_date,
            })
            .collect())
    }
}

#[derive(Serialize, Deserialize)]
struct Root1 {
    #[serde(rename = "data")]
    data: Vec<Data>,
}

#[derive(Serialize, Deserialize)]
struct Data {
    id: String,
    attributes: Attributes,
}

#[derive(Serialize, Deserialize)]
struct Attributes {
    subtype: String,
    #[serde(rename = "canonicalTitle")]
    canonical_title: String,
    titles: HashMap<String, String>,
    #[serde(rename = "posterImage")]
    poster_image: PosterImage,
    description: Option<String>,
    #[serde(rename = "startDate")]
    start_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PosterImage {
    tiny: Option<String>,
    large: Option<String>,
    small: Option<String>,
    medium: Option<String>,
    original: Option<String>,
}
