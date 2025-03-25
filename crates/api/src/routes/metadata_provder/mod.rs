mod link_entry;
mod list;
mod overview;
mod search;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/metadata-provider")
        .service(link_entry::register())
        .service(list::register())
        .service(search::register())
        .service(overview::register())
}
