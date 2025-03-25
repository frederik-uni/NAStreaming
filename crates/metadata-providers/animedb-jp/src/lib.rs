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

pub const ID: &'static str = "animedb-jp";

impl MetadataProvider for Instance {
    fn name(&self) -> &'static str {
        "アニメ大全"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Planed
    }

    fn origin(&self) -> &'static str {
        "https://animedb.jp"
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://db.animedb.jp/index.php/searchdata/?mode=view&id={id}")
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        None
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}
