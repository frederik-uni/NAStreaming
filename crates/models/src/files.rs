use std::path::PathBuf;

use storage_finder::{Cut, Episode, FileType, Kind, Resolutions, ThreeD};

pub type Value = ();
pub struct File {
    pub path: PathBuf,
    info: Info,
    pub file_type: FileType,
    pub ffprobe: Option<Value>,
    pub hash: Option<String>,
}

pub enum Info {
    Identified {
        group_id: String,
        res: Option<Resolutions>,
        kinds: Option<Kind>,
        extended: Option<Cut>,
        three_d: Option<ThreeD>,
    },
    Unidified {
        try_group: Option<String>,
        name: Vec<String>,
        ep_name: Vec<String>,
        season: Vec<u64>,
        episode: Vec<Episode>,
        year: Vec<u16>,
        resolutions: Vec<Resolutions>,
        three_ds: Vec<ThreeD>,
        extended: Vec<Cut>,
        kinds: Vec<Kind>,
        res: Vec<Resolutions>,
    },
}
