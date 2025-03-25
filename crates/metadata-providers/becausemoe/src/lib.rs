use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}

impl Instance {
    pub fn new(
        _data: HashMap<String, String>,
    ) -> Result<Box<dyn MetadataProvider + 'static>, String> {
        Ok(Box::new(Self {}))
    }
}

pub const ID: &'static str = "becausemoe";

impl MetadataProvider for Instance {
    fn name(&self) -> &'static str {
        "because.moe"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://because.moe"
    }

    fn id_to_url(&self, _id: &str) -> String {
        "https://because.moe".to_owned()
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
