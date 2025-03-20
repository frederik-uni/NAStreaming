use reqwest::{header::AUTHORIZATION, Error};
use serde::Deserialize;
use serde_json::json;

use crate::Instance;

impl Instance {
    pub fn search(&self, query: &str, year: Option<u16>, series: bool) -> Result<Vec<Data>, Error> {
        let url = self.server.join("/v4/search").unwrap();
        let mut body = json!({
            "query": query,
            "type": match series {
                true => "series",
                false => "movie"
            },
            "limit": 50,
            "offset": 0
        });

        if let Some(y) = year {
            body["year"] = json!(y);
        }
        let client: Root1 = self
            .client
            .post(url)
            .header(AUTHORIZATION, self.get_token()?)
            .json(&body)
            .send()?
            .json()?;
        Ok(client.data)
    }
}

#[derive(Deserialize)]
pub struct Root1 {
    data: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub name_translated: String,
    pub aliases: Vec<String>,
    pub title: String,
    pub year: String,
    pub remote_ids: Vec<RemoteId>,
    pub image_url: String,
    pub poster: String,
    pub thumbnail: String,
}

#[derive(Deserialize)]
pub struct RemoteId {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: i64,
    #[serde(rename = "sourceName")]
    pub source_name: String,
}
