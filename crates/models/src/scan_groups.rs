use crate::utils::DbUtils as _;
use crate::{table, DB};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealdb::{Error, RecordId};

table!(ScanGroup, "scan_groups");
#[derive(Serialize, Deserialize)]
pub struct ScanGroup {
    pub name: String,
    pub path: PathBuf,
    pub detect_path: Option<PathBuf>,
    pub display_order: Vec<String>,
    pub index_order: Vec<String>,
    pub series: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ScanGroupPaths {
    pub id: RecordId,
    pub path: PathBuf,
    pub detect_path: Option<PathBuf>,
}

impl ScanGroup {
    pub async fn find_by_path(path: &PathBuf) -> Result<Vec<(PathBuf, RecordId)>, Error> {
        let data: Vec<ScanGroupPaths> = DB
            .query(format!(
                "SELECT id, path, detect_path FROM {} WHERE path = $path OR detect_path = $path LIMIT 1",
                ScanGroup::table()
            ))
            .bind(("path", path.to_owned()))
            .await?
            .take(0)?;
        Ok(data
            .into_iter()
            .map(|v| match v.detect_path {
                None => vec![(v.path, v.id)],
                Some(p) => vec![(v.path, v.id.clone()), (p, v.id)],
            })
            .flatten()
            .collect())
    }
    pub async fn update_prefered_index_order(
        id: &str,
        prefered_index_order: Vec<String>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub async fn update_prefered_display_order(
        id: &str,
        prefered_display_order: Vec<String>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub async fn update_name(id: &str, name: String) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Deserialize)]
pub struct Id {
    pub id: RecordId,
}
