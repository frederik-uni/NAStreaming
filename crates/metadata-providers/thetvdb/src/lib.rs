mod login;
mod search;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use metadata_provider::MetadataProvider;
use reqwest::{blocking::Client, Url};

pub struct Instance {
    pub key: String,
    pub pin: String,
    pub access_token: Arc<Mutex<Option<String>>>,
    pub server: Url,
    pub client: Arc<Client>,
}

impl MetadataProvider for Instance {
    fn new(data: HashMap<String, String>) -> Result<Box<Self>, String> {
        let key = data.get("key").ok_or("No Key given".to_owned())?;
        let pin = data.get("pin").ok_or("No PIN given".to_owned())?;
        Ok(Box::new(Self {
            key: key.to_owned(),
            pin: pin.to_owned(),
            access_token: Default::default(),
            server: Url::parse("https://api4.thetvdb.com").unwrap(),
            client: Default::default(),
        }))
    }

    fn id() -> &'static str {
        "tvdb"
    }

    fn name(&self) -> &'static str {
        "TheTVDB.com"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://thetvdb.com"
    }

    fn search(&self) -> Option<Box<dyn metadata_provider::SearchProvider>> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
