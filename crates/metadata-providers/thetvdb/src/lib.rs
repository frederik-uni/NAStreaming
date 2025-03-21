mod info;
mod login;
mod search;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use metadata_provider::{
    fetcher::{Client, Url},
    MetadataProvider,
};

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
        "tv-db"
    }

    fn name(&self) -> &'static str {
        "TheTVDB.com"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::WIP
    }

    fn origin(&self) -> &'static str {
        "https://thetvdb.com"
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://thetvdb.com/dereferrer/series/{id}")
    }

    fn data_retrievel(&self) -> metadata_provider::DataRetrievel {
        metadata_provider::DataRetrievel::Api
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        Some(self)
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::read_to_string};

    use metadata_provider::MetadataProvider;
    use toml::Value;

    use crate::Instance;

    #[test]
    fn demo() {
        let data = read_to_string("../../../Config.toml").unwrap();
        let parsed: HashMap<String, HashMap<String, Value>> = toml::from_str(&data).unwrap();
        let map = parsed
            .get(Instance::id())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| v.as_str().map(|v| (k, v.to_owned())))
            .collect::<HashMap<_, _>>();
        let instance = Instance::new(map).expect("unreachable");
        let search_instance = instance.search().expect("unreachable");
        let result = search_instance
            .search("One piece", Some(1999), Some(true))
            .expect("Test failed");

        println!("{:#?}", result);
    }
}
