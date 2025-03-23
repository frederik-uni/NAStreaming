mod info;
mod search;

use std::collections::HashMap;

use metadata_provider::{fetcher::Client, Issue, MetadataProvider};

pub struct Instance {
    client: Client,
}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self {
            client: Default::default(),
        }))
    }
    fn id() -> &'static str {
        "anilist"
    }

    fn name(&self) -> &'static str {
        "AniList"
    }

    fn issues(&self) -> Vec<Issue> {
        vec![Issue::SeasonsAreSeperateEntries]
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::WIP
    }

    fn origin(&self) -> &'static str {
        "https://anilist.co/search/anime"
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        Some(self)
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://anilist.co/anime/{id}/")
    }
}

#[cfg(test)]
mod tests {
    use metadata_provider::MetadataProvider;

    use crate::Instance;

    #[tokio::test]
    async fn demo() {
        let instance = Instance::new(Default::default()).expect("unreachable");
        let search_instance = instance.search().expect("unreachable");
        let result = search_instance
            .search("One piece", Some(1999), None)
            .await
            .expect("Test failed");

        println!("{:#?}", result);
    }

    #[test]
    fn lookup() {
        let instance = Instance::new(Default::default()).expect("unreachable");
        instance.lookup("176301");
    }
}
