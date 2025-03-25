use metadata_provider::{
    fetcher::{reqwest::header::AUTHORIZATION, Client},
    Error,
};
use serde::Deserialize;
use serde_json::json;

use crate::Instance;

impl Instance {
    pub async fn lookup(&self, client: &Client, id: &str) -> Result<Root1, Error> {
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
        client
            .get(url)
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.get_token(client).await?),
            )
            .json(&json!({"short": false, "meta": "translations"}))
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(Deserialize)]
pub struct Root1 {}
