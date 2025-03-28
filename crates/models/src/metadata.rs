use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use structures::{Kind, Status};
use surrealdb::{opt::PatchOp, Datetime, Error};

use crate::{
    file_group::FileGroup,
    scan_groups::ScanGroup,
    table,
    utils::{Empty, RecordIdTyped},
    DB,
};

pub type Id = String;
pub type Timestamp = Duration;

table!(Country, "countries");
#[derive(Deserialize, Serialize)]
pub struct Country {
    name: String,
}

table!(Language, "languages");
#[derive(Deserialize, Serialize)]
pub struct Language {
    name: String,
}

table!(Entry, "entries");
#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub titles: HashMap<String, Vec<String>>,
    pub description: HashMap<String, String>,
    pub links: Vec<Source>,
    pub status: Status,
    pub release_dates: Vec<(RecordIdTyped<Country>, Timestamp)>,
    pub content_ratings: Vec<ContentRating>,
    pub categories: Vec<Id>,
    pub kind: Kind,
    pub original_country: Option<RecordIdTyped<Country>>,
    pub original_language: Option<RecordIdTyped<Language>>,
    pub spoken_language: Option<RecordIdTyped<Language>>,
    pub assets: Vec<Asset>,
    pub awards: Vec<Award>,
    /// key = Provider
    /// key.is_empty() = Manual user override
    pub updated: HashMap<String, Datetime>,
    pub created: Datetime,
    pub budget: Option<u64>,
    pub box_office: Vec<(RecordIdTyped<Country>, u64)>,
    pub geo_location: Vec<RecordIdTyped<Country>>,
    pub time_period: Id,
    pub characters: Vec<Cast>,
    pub companies: Companies,
    pub scan_group: RecordIdTyped<ScanGroup>,
    pub files: Vec<RecordIdTyped<FileGroup>>,
}

impl Entry {
    pub async fn add_file_group(
        id: RecordIdTyped<Entry>,
        group: RecordIdTyped<FileGroup>,
    ) -> Result<(), Error> {
        let _: Option<Empty> = DB
            .update(id.id())
            .patch(PatchOp::add("/files", group))
            .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Companies {
    pub studio: Vec<(RecordIdTyped<Country>, Id)>,
    pub network: Vec<(RecordIdTyped<Country>, Id)>,
    pub production: Vec<(RecordIdTyped<Country>, Id)>,
    pub distributor: Vec<(RecordIdTyped<Country>, Id)>,
    pub special_effects: Vec<(RecordIdTyped<Country>, Id)>,
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
    lang: Option<RecordIdTyped<Language>>,
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
    TvDB { series: bool, id: u64 },
    TheMovieDb(u64),
    CustomUrl(String),
    CustomIdk(String),
}

impl From<&String> for Source {
    fn from(value: &String) -> Self {
        match value.split_once("/") {
            Some((key, value)) => match key {
                "tv-db" => {
                    let (kind, id) = value.split_once("-").unwrap_or_default();
                    Self::TvDB {
                        series: kind == "series",
                        id: id.parse().unwrap_or_default(),
                    }
                }
                _ => Self::CustomUrl(value.to_owned()),
            },
            None => Self::CustomIdk(value.to_owned()),
        }
    }
}
