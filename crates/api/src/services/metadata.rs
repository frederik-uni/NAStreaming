use std::collections::HashMap;

use metadata_provider::{
    fetcher::Client,
    search::{Capabilities, SearchResult},
    DataRetrievel, MetadataProvider, State,
};

use crate::error::{ApiError, ApiResult};

pub struct MetadataService {
    high: Client,
    low: Client,
    services: HashMap<&'static str, Box<dyn MetadataProvider>>,
}

pub struct Provider {
    id: String,
    name: &'static str,
    origin: &'static str,
    state: State,
    data_retrievel: DataRetrievel,
    search: Option<Vec<Capabilities>>,
    info: bool,
}

impl MetadataService {
    pub fn providers(&self) -> Vec<Provider> {
        self.services
            .iter()
            .map(|(k, v)| Provider {
                id: k.to_string(),
                name: v.name(),
                origin: v.origin(),
                state: v.state(),
                data_retrievel: v.data_retrievel(),
                search: v.search().map(|v| v.capabilities()),
                info: v.info().is_some(),
            })
            .collect()
    }

    pub async fn search(
        &self,
        id: &str,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> ApiResult<Vec<SearchResult>> {
        Ok(self
            .services
            .get(id)
            .ok_or(ApiError::NotFoundInDb)?
            .search()
            .ok_or(ApiError::NotImplemented)?
            .search(&self.high, query, year, series)
            .await
            .unwrap())
    }

    pub fn new(mut maps: HashMap<String, HashMap<String, String>>) -> Result<Self, ApiError> {
        let high = Client::new();
        let low = high.clone();
        low.set_priority(10);
        let mut services: HashMap<&'static str, Box<(dyn MetadataProvider + 'static)>> =
            HashMap::new();
        let mut count = 0;

        macro_rules! create_instance {
            ($module:ident) => {
                services.insert(
                    $module::ID,
                    $module::Instance::new(maps.remove($module::ID).unwrap_or_default())
                        .expect("Failed to create metadata provider"),
                );
                count += 1;
            };
        }
        create_instance!(anemyfr);
        create_instance!(anidb);
        create_instance!(anilist);
        create_instance!(anime_characters_database);
        create_instance!(anime_news_network);
        create_instance!(anime_planet);
        create_instance!(animecountdown);
        create_instance!(animedb_jp);
        create_instance!(anisearch);
        create_instance!(becausemoe);
        create_instance!(fanart_tv);
        create_instance!(fandom);
        create_instance!(gamdb);
        create_instance!(imdb);
        create_instance!(kitsu);
        create_instance!(livechart);
        create_instance!(myanimelist);
        create_instance!(notify_moe);
        create_instance!(randomanime);
        create_instance!(rottentomatoes);
        create_instance!(simkl);
        create_instance!(themoviedb);
        create_instance!(thetvdb);
        create_instance!(trakt);
        assert_eq!(count, services.len());
        Ok(MetadataService {
            high,
            low,
            services,
        })
    }
}
