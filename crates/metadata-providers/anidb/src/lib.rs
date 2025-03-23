mod info;
mod search;

use std::collections::HashMap;

use metadata_provider::{
    fetcher::{Client, Url},
    Issue, MetadataProvider,
};
use scraper::Selector;

pub struct Instance {
    server: Url,
    client: Client,
    edit: Selector,
    first_aired_single: Selector,
    name_single: Selector,
    overview_single: Selector,
    img_single: Selector,
    kind_single: Selector,
    areas: Selector,
    a_multi: Selector,
    img_multi: Selector,
    kind_multi: Selector,
    first_aired_multi: Selector,
}

impl MetadataProvider for Instance {
    fn new(_data: HashMap<String, String>) -> Result<Box<Self>, String> {
        Ok(Box::new(Self { server: "http://api.anidb.net:9001/httpapi?request=anime&client=nastreamingclien&clientver=2&protover=1".parse().expect("Cant fail"), client: Default::default() ,
            areas: Selector::parse("tbody > tr").expect("failed to parse selector"),
            a_multi: Selector::parse("td.name.main.anime > a").expect("failed to parse selector"),
            first_aired_multi: Selector::parse("td.date.airdate").expect("failed to parse selector"),
            kind_multi: Selector::parse("td.type").expect("failed to parse selector"),
            img_multi: Selector::parse("td.thumb.anime > a > picture > img").expect("failed to parse selector"),
            edit: Selector::parse("#layout-main > div.g_content.anime_all.sidebar > div.g_section.info > div.edit_actions > span.modify.entry > a").expect("failed to parse selector"),
            name_single:
                Selector::parse("#tab_1_pane > div > table > tbody > tr.g_odd.romaji > td > span")
                    .expect("failed to parse selector"),
                kind_single:
                    Selector::parse("#tab_1_pane > div > table > tbody > tr.g_odd.type > td")
                        .expect("failed to parse selector"),
            first_aired_single:
                Selector::parse("#tab_1_pane > div > table > tbody > tr.g_odd.year > td > span")
                    .expect("failed to parse selector"),
            img_single: Selector::parse("#layout-main > div.g_content.anime_all.sidebar > div.g_section.info > div.block > div.image > div > picture > img").expect("failed to parse selector"),
            overview_single: Selector::parse(
                "#layout-main > div.g_content.anime_all.sidebar > div.g_bubble.g_section.desc",
            )
            .expect("failed to parse selector"),
        }))
    }
    fn id() -> &'static str {
        "anidb"
    }

    fn name(&self) -> &'static str {
        "AniDB"
    }

    fn issues(&self) -> Vec<Issue> {
        vec![Issue::SeasonsAreSeperateEntries]
    }

    fn id_to_url(&self, id: &str) -> String {
        format!("https://anidb.net/anime/{id}")
    }

    fn state(&self) -> metadata_provider::State {
        metadata_provider::State::WIP
    }

    fn data_retrievel(&self) -> metadata_provider::DataRetrievel {
        metadata_provider::DataRetrievel::SearchScraperInfoApi
    }

    fn origin(&self) -> &'static str {
        "https://anidb.net"
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
    use metadata_provider::MetadataProvider;

    use crate::Instance;

    #[tokio::test]
    async fn demo() {
        let instance = Instance::new(Default::default()).expect("unreachable");
        let search_instance = instance.search().expect("unreachable");
        let result = search_instance
            .search("One piece", Some(1992), None)
            .await
            .expect("Test failed");

        println!("{:#?}", result);
    }
}
