use metadata_provider::{
    async_trait,
    fetcher::{Client, Url},
    search::{Capabilities, SearchProvider, SearchResult},
    Error,
};

use crate::Instance;

#[async_trait]
impl SearchProvider for Instance {
    async fn search(
        &self,
        client: &Client,
        query: &str,
        year: Option<u16>,
        _series: Option<bool>,
    ) -> Result<Vec<SearchResult>, Error> {
        let mut url = Url::parse("https://anidb.net/anime/").expect("url shouldnt fail");
        url.query_pairs_mut()
            .append_pair("adb.search", query)
            .append_pair("noalias", "1")
            .append_pair("do.update", "Search");
        if let Some(year) = year {
            url.query_pairs_mut()
                .append_pair("season.year", &year.to_string());
        }
        let document = client.get(url).with_user_agent().send().await?.html();

        if let Some(edit) = document.select(&self.edit).next() {
            return Ok(vec![SearchResult {
                id: edit
                    .attr("href")
                    .ok_or(Error::NodeNotFound)?
                    .split("/")
                    .collect::<Vec<_>>()[1]
                    .to_string(),
                cover: Some(
                    document
                        .select(&self.img_single)
                        .next()
                        .ok_or(Error::NodeNotFound)?
                        .attr("src")
                        .ok_or(Error::NodeNotFound)?
                        .to_string(),
                ),
                names: vec![document
                    .select(&self.name_single)
                    .next()
                    .ok_or(Error::NodeNotFound)?
                    .text()
                    .collect::<String>()],
                overview: Some(
                    document
                        .select(&self.overview_single)
                        .next()
                        .ok_or(Error::NodeNotFound)?
                        .text()
                        .collect::<String>(),
                ),
                first_aired: Some(
                    document
                        .select(&self.first_aired_single)
                        .next()
                        .ok_or(Error::NodeNotFound)?
                        .inner_html(),
                ),
                kind: Some(
                    document
                        .select(&self.kind_single)
                        .next()
                        .ok_or(Error::NodeNotFound)?
                        .inner_html()
                        .trim()
                        .to_owned(),
                ),
            }]);
        }

        document
            .select(&self.areas)
            .map(|area| {
                let a = area
                    .select(&self.a_multi)
                    .next()
                    .ok_or(Error::NodeNotFound)?;
                Ok(SearchResult {
                    id: a
                        .attr("href")
                        .ok_or(Error::NodeNotFound)?
                        .replace("/anime/", ""),
                    cover: Some(
                        area.select(&self.img_multi)
                            .next()
                            .ok_or(Error::NodeNotFound)?
                            .attr("src")
                            .ok_or(Error::NodeNotFound)?
                            .to_string(),
                    ),
                    names: vec![a.inner_html()],
                    overview: None,
                    first_aired: Some(
                        area.select(&self.first_aired_multi)
                            .next()
                            .ok_or(Error::NodeNotFound)?
                            .inner_html()
                            .trim()
                            .to_owned(),
                    ),
                    kind: Some(
                        area.select(&self.kind_multi)
                            .next()
                            .ok_or(Error::NodeNotFound)?
                            .inner_html()
                            .trim()
                            .to_owned(),
                    ),
                })
            })
            .collect()
    }

    fn capabilities(&self) -> Vec<Capabilities> {
        vec![Capabilities::TitleExact, Capabilities::Year]
    }
}
