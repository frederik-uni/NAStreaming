use std::collections::HashMap;

use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use models::{
    metadata::{Companies, Entry, Source},
    scan_groups::ScanGroup,
    DbUtils,
};
use structures::{entry::AddEntryRequest, Kind, Status};

use crate::error::ApiResult;

#[api_operation(
    tag = "entry",
    summary = "Adds a Series/Movie",
    description = r###""###
)]
async fn exec(Json(data): Json<AddEntryRequest>) -> ApiResult<CreatedJson<String>> {
    let ids: Vec<_> = data.ids.iter().map(Source::from).collect();
    let id = Entry {
        titles: HashMap::new(),
        description: HashMap::new(),
        links: ids,
        status: Status::Unknown,
        release_dates: Vec::new(),
        content_ratings: Vec::new(),
        categories: Vec::new(),
        kind: match data.series {
            true => Kind::Series,
            false => Kind::Movie,
        },
        original_country: None,
        original_language: None,
        spoken_language: None,
        assets: Vec::new(),
        awards: Vec::new(),
        updated: [("".to_owned(), Default::default())].into_iter().collect(),
        created: Default::default(),
        budget: None,
        box_office: Vec::new(),
        geo_location: Vec::new(),
        time_period: String::new(),
        characters: Vec::new(),
        companies: Companies {
            studio: Vec::new(),
            network: Vec::new(),
            production: Vec::new(),
            distributor: Vec::new(),
            special_effects: Vec::new(),
        },
        scan_group: ScanGroup::to_id(&data.scan_group_id),
        files: vec![],
    }
    .add()
    .await?;
    //TODO: start fetch metadata service. callback => start linking
    Ok(CreatedJson(id.id.key().to_string()))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/add").route(apistos::web::put().to(exec))
}
