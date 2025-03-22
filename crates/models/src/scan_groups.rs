use crate::table;
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

impl ScanGroup {
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
