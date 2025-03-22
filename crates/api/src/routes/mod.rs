mod entry;
mod episode;
mod file;
mod home;
mod init;
mod lib;
mod management;
mod metadata_provder;
mod user;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/v1")
        .service(home::register())
        .service(user::register())
        .service(file::register())
        .service(episode::register())
        .service(lib::register())
        .service(entry::register())
        .service(metadata_provder::register())
        .service(init::register())
}
