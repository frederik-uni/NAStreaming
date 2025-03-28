use serde::{Deserialize, Serialize};
use storage_finder::Episode;
use surrealdb::opt::PatchOp;
use surrealdb::{Error, RecordId};

use crate::utils::{DbUtils as _, Empty};
use crate::{files::File, metadata::Entry, table, utils::RecordIdTyped, DB};

table!(FileGroup, "file_groups");
#[derive(Deserialize, Serialize)]
pub struct FileGroup {
    pub info: Option<SeasonEpisode>,
    pub files: Vec<RecordIdTyped<File>>,
    pub entry: RecordIdTyped<Entry>,
}

impl FileGroup {
    pub async fn update_or_create(
        entry: RecordIdTyped<Entry>,
        info: Option<SeasonEpisode>,
        file: RecordIdTyped<File>,
    ) -> Result<RecordId, Error> {
        if let Some(info) = &info {
            let data: Vec<Empty> = DB
                .query(format!(
                    "SELECT * FROM ((SELECT files FROM {})[0].files) WHERE info = $info LIMIT 1",
                    entry.id()
                ))
                .bind(("entry", entry.clone()))
                .bind(("info", info.clone()))
                .await?
                .take(0)?;
            if let Some(add) = data.get(0) {
                let _: Option<Empty> = DB
                    .update(&add.id)
                    .patch(PatchOp::add("/files", file))
                    .await?;
                return Ok(add.id.clone());
            }
        }
        let id = Self {
            info,
            files: vec![file],
            entry: entry.clone(),
        }
        .add()
        .await?
        .id;
        Entry::add_file_group(entry, id.clone().into()).await?;
        Ok(id)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SeasonEpisode {
    pub season: u64,
    pub episode: Option<Episode>,
}
