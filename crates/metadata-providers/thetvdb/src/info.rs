use metadata_provider::{fetcher::reqwest::header::AUTHORIZATION, Error};
use serde::Deserialize;
use serde_json::json;

use crate::Instance;

impl Instance {
    pub fn lookup(&self, id: &str) -> Result<Root1, Error> {
        let (kind, id) = id.split_once("-").ok_or(Error::InvalidId)?;
        let url = self
            .server
            .join(&format!(
                "/{}/{id}/extended",
                match kind {
                    "series" => "series",
                    _ => "movies",
                }
            ))
            .unwrap();

        //TODO:
        //series/{id}/episodes/{season-type}
        //seasons/{id}/extended
        self.client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.get_token()?))
            .json(&json!({"short": false, "meta": "translations"}))
            .send()?
            .json()
    }
}

#[derive(Deserialize)]
pub struct Root1 {}
