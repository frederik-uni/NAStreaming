use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use structures::{Kind, Status};

pub type Id = String;
pub type LocationId = String;
pub type LanguageId = String;
pub type Timestamp = Duration;

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub titles: HashMap<LanguageId, Vec<String>>,
    pub description: HashMap<LanguageId, String>,
    pub links: Vec<Source>,
    pub status: Status,
    pub release_dates: Vec<(LocationId, Timestamp)>,
    pub content_ratings: Vec<ContentRating>,
    pub categories: Vec<Id>,
    pub kind: Kind,
    pub original_country: LocationId,
    pub original_language: LanguageId,
    pub spoken_language: LanguageId,
    pub assets: Vec<Asset>,
    pub awards: Vec<Award>,
    pub updated: u64,
    pub created: u64,
    pub budget: u64,
    pub box_office: Vec<(LocationId, u64)>,
    pub geo_location: Vec<LocationId>,
    pub time_period: Id,
    pub characters: Vec<Cast>,
    pub companies: Companies,
}

#[derive(Deserialize, Serialize)]
pub struct Companies {
    pub studio: Vec<(LocationId, Id)>,
    pub network: Vec<(LocationId, Id)>,
    pub production: Vec<(LocationId, Id)>,
    pub distributor: Vec<(LocationId, Id)>,
    pub special_effects: Vec<(LocationId, Id)>,
}
#[derive(Deserialize, Serialize)]
pub struct Cast {
    pub name: String,
    pub person: Id,
    pub role: CastRole,
}

#[derive(Deserialize, Serialize)]
pub enum CastRole {
    Cast,
    Director,
    Writer,
    Producer,
}

#[derive(Deserialize, Serialize)]
pub struct Award {
    won: bool,
    year: u16,
    award: Id,
}

#[derive(Deserialize, Serialize)]
pub struct Asset {
    lang: Option<LanguageId>,
    source: String,
    kind: AssetKind,
}

#[derive(Deserialize, Serialize)]
pub enum AssetKind {
    Trailer,
    Background,
    Banner,
    Cover,
    ClearArt,
    ClearLogo,
    Cinemagraph,
    Icon,
    Poster,
    Art,
}

#[derive(Deserialize, Serialize)]
pub enum ContentRating {
    USA(ContentRatingUSA),
    Brazil(u8),
}

#[derive(Deserialize, Serialize)]
pub enum ContentRatingUSA {
    G,
    PG,
    PG13,
    R,
    NC17,
}

#[derive(Deserialize, Serialize)]
pub enum Source {
    TheTvDb(u64),
    TheMovieDb(u64),
    CustomUrl(String),
}
