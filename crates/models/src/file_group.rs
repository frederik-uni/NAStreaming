use storage_finder::Episode;

pub struct FileGroup {
    pub entry_id: String,
    pub season: u64,
    pub episode: Option<Episode>,
    pub files: Vec<String>,
}
