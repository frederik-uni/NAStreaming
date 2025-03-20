use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    models::connect().await.expect("Error connecting to DB");
    let _ = models::scan_groups::ScanGroup {
        name: "Anime".to_string(),
        path: dirs::home_dir()
            .expect("Failed to get home directory")
            .join("movie_files"),
        detect_path: None,
        display_order: vec![],
        index_order: vec![],
        series: true,
    }
    .add()
    .await
    .expect("Error adding movies");
    let all = models::scan_groups::ScanGroup::all()
        .await
        .expect("Error getting all groups");
    let mut scan = VecDeque::new();
    for item in all {
        if let Some(detect_path) = item.detect_path {
            scan.push_front((detect_path, item.series))
        }
        scan.push_back((item.path, item.series))
    }

    for (path, _) in scan {
        let items = storage_finder::parse_library(&path, &Default::default());
    }
}
