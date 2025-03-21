use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self {}))
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

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
