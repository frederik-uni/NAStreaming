use actix_web::web::Json;
use apistos::{actix::CreatedJson, api_operation};
use models::{
    file_group::{FileGroup, SeasonEpisode},
    files::{File, Info},
    metadata::Entry,
    DbUtils,
};
use storage_finder::{Cut, Episode, Kind, ParseBigDecimalError, Resolutions, ThreeD};
use structures::file::SetEntryRequest;

use crate::error::ApiResult;

#[api_operation(
    tag = "file",
    summary = "Links file to Entry(Movie/Series)",
    description = r###""###
)]
async fn exec(Json(data): Json<SetEntryRequest>) -> ApiResult<CreatedJson<u16>> {
    let entry = Entry::to_id(&data.entry_id);
    for item in data.items {
        let episode = item
            .episode
            .map(|v| {
                Ok::<_, ParseBigDecimalError>(match v {
                    structures::Episode::List(items) => Episode::List(
                        items
                            .into_iter()
                            .map(|v| v.parse())
                            .collect::<Result<_, _>>()?,
                    ),
                    structures::Episode::Single(s) => Episode::Single(s.parse()?),
                    structures::Episode::Part(p1, p2) => Episode::Part(p1.parse()?, p2),
                    structures::Episode::RangeInclusive(s, e) => {
                        Episode::Part(s.parse()?, e.parse()?)
                    }
                })
            })
            .transpose()?;
        let file = File::to_id(&item.file_id);
        let gid = FileGroup::update_or_create(
            entry.clone(),
            item.season.map(|v| SeasonEpisode {
                season: v as u64,
                episode,
            }),
            file.clone(),
        )
        .await?;
        File::update_info(
            file,
            Info::Identified {
                group_id: gid.into(),
                resolution: item
                    .resolution
                    .map(Resolutions::try_from)
                    .transpose()
                    .unwrap_or_default(),
                kind: item
                    .kind
                    .map(Kind::try_from)
                    .transpose()
                    .unwrap_or_default(),
                extended: item
                    .extended
                    .map(Cut::try_from)
                    .transpose()
                    .unwrap_or_default(),
                three_d: item
                    .three_d
                    .map(ThreeD::try_from)
                    .transpose()
                    .unwrap_or_default(),
            },
        )
        .await?;
    }
    //TODO: start linking
    Ok(CreatedJson(201))
}

pub fn register() -> apistos::web::Resource {
    apistos::web::resource("/link-entry").route(apistos::web::put().to(exec))
}
