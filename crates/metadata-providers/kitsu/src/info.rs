use crate::Instance;
use metadata_provider::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Instance {
    pub async fn lookup(&self, id: &str) -> Result<Root1, Error> {
        self.client
            .get(format!("https://kitsu.app/api/edge/anime/{id}"))
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(Serialize, Deserialize)]
pub struct Root1 {
    data: Data,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    id: String,
    data_type: String,
    links: DataLinks,
    attributes: Attributes,
    relationships: HashMap<String, Relationship>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    created_at: String,
    updated_at: String,
    slug: String,
    synopsis: String,
    description: String,
    cover_image_top_offset: i64,
    titles: HashMap<String, String>,
    canonical_title: String,
    abbreviated_titles: Vec<String>,
    average_rating: String,
    rating_frequencies: HashMap<String, String>,
    user_count: i64,
    favorites_count: i64,
    start_date: String,
    end_date: String,
    next_release: Option<()>,
    popularity_rank: i64,
    rating_rank: i64,
    age_rating: String,
    age_rating_guide: String,
    subtype: String,
    status: String,
    tba: Option<()>,
    poster_image: PosterImage,
    cover_image: CoverImage,
    episode_count: i64,
    episode_length: i64,
    total_length: i64,
    youtube_video_id: String,
    show_type: String,
    nsfw: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CoverImage {
    tiny: String,
    large: String,
    small: String,
    original: String,
}

#[derive(Serialize, Deserialize)]
pub struct PosterImage {
    tiny: String,
    large: String,
    small: String,
    medium: String,
    original: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataLinks {
    #[serde(rename = "self")]
    links_self: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relationship {
    links: RelationshipLinks,
}

#[derive(Serialize, Deserialize)]
pub struct RelationshipLinks {
    #[serde(rename = "self")]
    links_self: String,
    related: String,
}
