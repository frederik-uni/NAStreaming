mod set_entry;
mod set_episode;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/file")
        .service(set_entry::register())
        .service(set_episode::register())
}
