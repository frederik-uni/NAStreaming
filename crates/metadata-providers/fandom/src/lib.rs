mod search;

use std::collections::HashMap;

use metadata_provider::{fetcher::Client, DataRetrievel, MetadataProvider};
use scraper::Selector;

pub struct Instance {
    pub client: Client,
    pub articles: Selector,
    pub a: Selector,
    pub desc: Selector,
    pub kind: Selector,
    pub cover: Selector,
}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self {
            articles: Selector::parse("#mw-content-text > section > div > ul > li").expect("selector hardcoded"),
            a:Selector::parse("article > div.unified-search__result__community__content > a")
                .expect("selector hardcoded"),
            desc: Selector::parse("article > div.unified-search__result__community__content > div.unified-search__result__community__description").expect("selector hardcoded"),
            cover:
                Selector::parse("article > div.unified-search__result__community__thumbnail > a > img")
                    .expect("selector hardcoded"),
                kind: Selector::parse("article > div.unified-search__result__community__content > div.unified-search__result__community__content__hub").expect("selector hardcoded"),
            client: Default::default(),
        }))
    }
    fn id() -> &'static str {
        "fandom"
    }

    fn name(&self) -> &'static str {
        "Fandom"
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::Running
    }

    fn data_retrievel(&self) -> DataRetrievel {
        DataRetrievel::Scraping
    }

    fn origin(&self) -> &'static str {
        "https://www.fandom.com"
    }

    fn id_to_url(&self, id: &str) -> String {
        id.to_owned()
    }

    fn search(&self) -> Option<&dyn metadata_provider::search::SearchProvider> {
        Some(self as &dyn metadata_provider::search::SearchProvider)
    }

    fn info(&self) -> Option<Box<dyn metadata_provider::InfoProvider>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use metadata_provider::MetadataProvider;

    use crate::Instance;

    #[test]
    fn demo() {
        let instance = Instance::new(Default::default()).expect("unreachable");
        let search_instance = instance.search().expect("unreachable");

        let result = search_instance
            .search("One piece", None, None)
            .expect("Test failed");

        println!("{:#?}", result);
    }
}
