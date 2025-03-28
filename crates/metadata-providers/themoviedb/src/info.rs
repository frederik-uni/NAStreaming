use metadata_provider::{
    fetcher::{reqwest::header::AUTHORIZATION, Client},
    Error,
};
use serde::Deserialize;

use crate::Instance;

impl Instance {
    pub async fn lookup(&self, client: &Client, id: &str) -> Result<Root1, Error> {
        let (kind, id) = id.split_once('-').ok_or(Error::InvalidId)?;
        client
            .get(format!("https://api.themoviedb.org/3/{kind}/{id}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(Deserialize)]
pub struct Root1 {}
