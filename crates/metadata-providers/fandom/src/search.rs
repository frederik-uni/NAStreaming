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
        _year: Option<u16>,
        _series: Option<bool>,
    ) -> Result<Vec<SearchResult>, Error> {
        let mut url: Url = "https://community.fandom.com/wiki/Special:SearchCommunity"
            .parse()
            .expect("url shouldnt fail");
        url.query_pairs_mut().append_pair("query", query);
        let req = client.get(url).send().await?;
        let document = req.html();
        let data = document
            .select(&self.articles)
            .map(|article| {
                let a = article.select(&self.a).next().ok_or(Error::NodeNotFound)?;
                let url = a.attr("href").ok_or(Error::NodeNotFound)?;
                let kind = article
                    .select(&self.kind)
                    .next()
                    .ok_or(Error::NodeNotFound)?
                    .inner_html()
                    .trim()
                    .to_owned();
                if url.contains("https://www.fandom.com") || url.is_empty() {
                    Ok(None::<SearchResult>)
                } else {
                    Ok(Some(SearchResult {
                        id: url.to_string(),
                        cover: Some(
                            article
                                .select(&self.cover)
                                .next()
                                .ok_or(Error::NodeNotFound)?
                                .attr("src")
                                .ok_or(Error::NodeNotFound)?
                                .to_string(),
                        ),
                        names: vec![a.inner_html().trim().to_owned()],
                        overview: Some(
                            article
                                .select(&self.desc)
                                .next()
                                .ok_or(Error::NodeNotFound)?
                                .inner_html()
                                .trim()
                                .to_string(),
                        ),
                        first_aired: None,
                        kind: Some(kind),
                    }))
                }
            })
            .collect::<Result<Vec<Option<SearchResult>>, Error>>()?;
        Ok(data.into_iter().filter_map(|v| v).collect())
    }

    fn capabilities(&self) -> Vec<Capabilities> {
        vec![]
    }
}
