use serde::{Deserialize, Serialize};
use storage_finder::Episode;

use crate::{metadata::Entry, utils::RecordIdTyped};

#[derive(Deserialize, Serialize)]
pub struct FileGroup {
    pub entry: RecordIdTyped<Entry>,
    pub season: u64,
    pub episode: Option<Episode>,
    pub files: Vec<RecordIdTyped<Entry>>,
}
