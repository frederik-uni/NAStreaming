use std::collections::HashMap;

use metadata_provider::MetadataProvider;

pub struct Instance {}

pub const ID: &'static str = "anemyfr";

impl Instance {
    pub fn new(
        _data: HashMap<String, String>,
    ) -> Result<Box<dyn MetadataProvider + 'static>, String> {
        Ok(Box::new(Self {}))
    }
}

impl MetadataProvider for Instance {
    fn name(&self) -> &'static str {
        "Anemy"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://anemy.fr"
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://anemy.fr/anime.php?id={id}")
    }
}
