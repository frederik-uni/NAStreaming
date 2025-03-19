use crate::DB;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealdb::RecordId;

#[derive(Serialize, Deserialize)]
pub struct ScanGroup {
    pub name: String,
    pub path: PathBuf,
    pub detect_path: Option<PathBuf>,
    pub display_order: Vec<String>,
    pub index_order: Vec<String>,
    pub series: bool,
}

#[derive(Deserialize)]
pub struct Id {
    pub id: RecordId,
}
impl ScanGroup {
    pub fn name() -> &'static str {
        "scan_group"
    }
    pub async fn add(self) -> surrealdb::Result<RecordId> {
        let mut v:Vec<Id> = DB.insert(Self::name()).content(self).await?;
        Ok(v.remove(0).id)
    }

    pub async fn all() -> surrealdb::Result<Vec<Self>> {
        DB.select(Self::name()).await
    }
}
