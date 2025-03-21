mod search;

use std::collections::HashMap;

use metadata_provider::{fetcher::Client, MetadataProvider};

pub struct Instance {
    client: Client,
    key: String,
}

impl MetadataProvider for Instance {
    fn new(data: HashMap<String, String>) -> Result<Box<Self>, String> {
        let key = data.get("key").ok_or("No key given".to_owned())?.to_owned();

        Ok(Box::new(Self {
            key,
            client: Default::default(),
        }))
    }
    fn id() -> &'static str {
        "fanart-tv"
    }

    fn name(&self) -> &'static str {
        "FanArt"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::WIP
    }

    fn origin(&self) -> &'static str {
        "https://fanart.tv"
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://fanart.tv/series/{id}")
    }

    fn data_retrievel(&self) -> metadata_provider::DataRetrievel {
        metadata_provider::DataRetrievel::SearchInternalApiInfoApi
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
            .search("One piece", None, Some(true))
            .expect("Test failed");

        println!("{:#?}", result);
    }
}
