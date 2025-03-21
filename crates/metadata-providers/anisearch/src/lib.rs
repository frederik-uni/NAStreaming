use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self {}))
    }
    fn id() -> &'static str {
        "anisearch"
    }

    fn name(&self) -> &'static str {
        "aniSearch"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://www.anisearch.de"
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://www.anisearch.de/anime/{id}")
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
