use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use storage_finder::{Cut, Entry, Episode, FileType, Kind, Resolutions, ThreeD};
use surrealdb::Error;

use crate::file_group::FileGroup;
use crate::utils::DbUtils;
use crate::DB;
use crate::{table, utils::RecordIdTyped};

pub type Value = ();
table!(File, "files");
#[derive(Deserialize, Serialize)]
pub struct File {
    pub path: PathBuf,
    pub info: Info,
    pub linked: bool,
    pub file_type: FileType,
    pub ffprobe: Option<Value>,
    pub hash: Option<String>,
}

impl File {
    pub async fn find_related(
        path: PathBuf,
    ) -> Result<Option<RecordIdTyped<crate::metadata::Entry>>, Error> {
        let query = format!("(SELECT info FROM {} WHERE linked = true AND string::starts_with(path, $group_path) LIMIT 1)[0].info.group", Self::table());

        let data: Option<RecordIdTyped<FileGroup>> =
            DB.query(query).bind(("group_path", path)).await?.take(0)?;
        match data {
            Some(data) => {
                let item = data.get().await?.unwrap();
                Ok(Some(item.data.entry))
            }
            None => Ok(None),
        }
    }
    pub async fn add_entries(entries: Vec<Entry>) -> Result<Option<()>, Error> {
        let mut insert = vec![];
        let mut cache: HashMap<PathBuf, Option<RecordIdTyped<crate::metadata::Entry>>> =
            HashMap::new();
        for v in entries {
            let mut group_path = v.root_path.clone();
            if let Some(v) = v.path.components().next() {
                group_path = group_path.join(v);
            }
            let related = match cache.get(&group_path) {
                Some(v) => v.clone(),
                None => {
                    let related = Self::find_related(group_path.clone()).await?;
                    cache.insert(group_path, related.clone());
                    related
                }
            };
            insert.push(File {
                path: v.path,
                info: Info::Unidified {
                    try_group: related,
                    name: v.name,
                    ep_name: v.ep_name,
                    sure: v.sure,
                    season: v.season,
                    episode: v.episode,
                    year: v.year,
                    resolutions: v.resolutions,
                    three_ds: v.three_ds,
                    extended: v.extended,
                    kinds: v.kinds,
                },
                file_type: v.file_type,
                ffprobe: None,
                hash: None,
                linked: false,
            });
        }
        File::add_bulk(insert).await?;

        Ok(Some(()))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Info {
    Identified {
        group_id: String,
        res: Option<Resolutions>,
        kinds: Option<Kind>,
        extended: Option<Cut>,
        three_d: Option<ThreeD>,
    },
    Unidified {
        try_group: Option<RecordIdTyped<crate::metadata::Entry>>,
        name: Vec<String>,
        ep_name: Vec<String>,
        sure: bool,
        season: Vec<u64>,
        episode: Vec<Episode>,
        year: Vec<u16>,
        resolutions: Vec<Resolutions>,
        three_ds: Vec<ThreeD>,
        extended: Vec<Cut>,
        kinds: Vec<Kind>,
    },
}
