use std::process::Command;

use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("frederik-uni")
        .repo_name("NAStreaming")
        .bin_name("github")
        .no_confirm(true)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    log::info!("UpToDate: `{}`!", status.uptodate());
    if !status.uptodate() {
        restart();
    }
    Ok(())
}

pub fn restart() {
    if let (Ok(currend_dir), Ok(current_exe)) = (std::env::current_dir(), std::env::current_exe()) {
        log::info!("Restarting Application");
        let _ = Command::new(current_exe).current_dir(currend_dir).spawn();
        std::process::exit(0);
    } else {
        log::warn!("Failed to restart the application");
    }
}
