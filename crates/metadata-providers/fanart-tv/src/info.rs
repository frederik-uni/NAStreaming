use metadata_provider::{fetcher::Client, Error};
use serde::{Deserialize, Serialize};

use crate::Instance;

impl Instance {
    pub async fn lookup(&self, client: &Client, id: &str) -> Result<Root1, Error> {
        let (kind, id) = id.split_once("-").unwrap_or_default();
        let url = format!(
            "http://webservice.fanart.tv/v3/{}/{}?api_key={}",
            kind, id, self.key
        );
        let data: Root1 = client.get(url).send().await?.json().await?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Art {
    id: String,
    lang: String,
    likes: String,
    url: String,
    season: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct Root1 {
    characterart: Vec<Art>,
    clearart: Vec<Art>,
    clearlogo: Vec<Art>,
    hdclearart: Vec<Art>,
    hdtvlogo: Vec<Art>,
    name: String,
    seasonbanner: Vec<Art>,
    seasonposter: Vec<Art>,
    seasonthumb: Vec<Art>,
    showbackground: Vec<Art>,
    thetvdb_id: String,
    tvbanner: Vec<Art>,
    tvposter: Vec<Art>,
    tvthumb: Vec<Art>,
}
