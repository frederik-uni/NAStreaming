use metadata_provider::{fetcher::reqwest::header::AUTHORIZATION, Error};
use serde::Deserialize;

use crate::Instance;

impl Instance {
    pub fn lookup(&self, id: &str) -> Result<Root1, Error> {
        let (kind, id) = id.split_once('-').ok_or(Error::InvalidId)?;
        self.client
            .get(format!("https://api.themoviedb.org/3/{kind}/{id}"))
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .send()?
            .json()
    }
}

#[derive(Deserialize)]
pub struct Root1 {}
