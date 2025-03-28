use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use storage_finder::{Cut, Entry, Episode, FileType, Kind, Resolutions, ThreeD};
use surrealdb::opt::PatchOp;
use surrealdb::{Error, RecordId};

use crate::file_group::FileGroup;
use crate::utils::{DbUtils, Empty};
use crate::{table, utils::RecordIdTyped};
use crate::{Record, DB};

pub type Value = ();
table!(File, "files");
#[derive(Deserialize, Serialize)]
pub struct File {
    pub root_path: PathBuf,
    pub path: PathBuf,
    pub info: Info,
    pub linked: bool,
    pub file_type: FileType,
    pub ffprobe: Option<Value>,
    pub hash: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FileValidate {
    pub root_path: PathBuf,
    pub path: PathBuf,
    pub info: Unidentified,
}

#[derive(Deserialize, Serialize)]
pub struct FilePath {
    pub id: RecordId,
    pub root_path: PathBuf,
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct Grouped {
    gpath: PathBuf,
    ids: Vec<RecordId>,
}

impl File {
    pub async fn update_info(id: RecordIdTyped<File>, info: Info) -> Result<(), Error> {
        let _: Option<Empty> = DB
            .update(id.id())
            .patch(PatchOp::replace("/info", info))
            .await?;
        let _: Option<Empty> = DB
            .update(id.id())
            .patch(PatchOp::replace("/linked", true))
            .await?;
        Ok(())
    }
    pub async fn paths() -> Result<Vec<PathBuf>, Error> {
        DB.query(format!(
            "(SELECT root_path + path AS full_path FROM {}).full_path",
            Self::table()
        ))
        .await?
        .take(0)
    }

    pub async fn group() -> Result<HashMap<PathBuf, Vec<RecordId>>, Error> {
        let items: Vec<Grouped> = DB
            .query(format!(
                "
                SELECT
                    array::group(id) AS ids,
                    root_path + \"/\" + array::first(string::split(path, \"/\")) AS gpath
                FROM {}
                WHERE linked = false
                GROUP BY gpath;
                ",
                Self::table()
            ))
            .await?
            .take(0)?;

        Ok(items.into_iter().map(|v| (v.gpath, v.ids)).collect())
    }

    pub async fn get_info(items: Vec<String>) -> Result<Vec<Record<FileValidate>>, Error> {
        Ok(DB
            .query(format!(
                "SELECT id, root_path, path, info FROM $ids WHERE linked = false"
            ))
            .bind((
                "ids",
                items
                    .into_iter()
                    .map(|v| RecordId::from((Self::table(), v.as_str())))
                    .collect::<Vec<_>>(),
            ))
            .await?
            .take(0)?)
    }

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
        //TODO: check deleted
        let mut insert = vec![];
        for v in entries {
            insert.push(File {
                path: v.path,
                root_path: v.root_path,
                info: Info::Unidified(Unidentified {
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
                }),
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
        group_id: RecordIdTyped<FileGroup>,
        resolution: Option<Resolutions>,
        kind: Option<Kind>,
        extended: Option<Cut>,
        three_d: Option<ThreeD>,
    },
    Unidified(Unidentified),
}

#[derive(Serialize, Deserialize)]
pub struct Unidentified {
    pub name: Vec<String>,
    pub ep_name: Vec<String>,
    pub sure: bool,
    pub season: Vec<u64>,
    pub episode: Vec<Episode>,
    pub year: Vec<u16>,
    pub resolutions: Vec<Resolutions>,
    pub three_ds: Vec<ThreeD>,
    pub extended: Vec<Cut>,
    pub kinds: Vec<Kind>,
}
