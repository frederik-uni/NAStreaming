use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}
impl Instance {
    pub fn new(
        data: HashMap<String, String>,
    ) -> Result<Box<dyn MetadataProvider + 'static>, String> {
        Ok(Box::new(Self {}))
    }
}

pub const ID: &'static str = "trakt";

impl MetadataProvider for Instance {
    fn name(&self) -> &'static str {
        "Trakt"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://trakt.tv"
    }

    fn id_to_url(&self, id: &str) -> String {
        let (kind, id) = id.split_once("-").unwrap_or_default();
        format!("https://trakt.tv/{kind}/{id}")
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
