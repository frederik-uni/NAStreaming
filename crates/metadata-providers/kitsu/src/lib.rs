use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self {}))
    }
    fn id() -> &'static str {
        "kitsu"
    }

    fn name(&self) -> &'static str {
        "Kitsu"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://kitsu.app/explore/anime"
    }

    fn search(&self) -> Option<Box<dyn metadata_provider::SearchProvider>> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
