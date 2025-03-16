use std::path::PathBuf;

pub struct File {
    path: PathBuf,
    season: i64,
    episode: Option<String>,
    name: String,
    //TODO: add more info
}
