mod link_entry;
mod list;
mod search;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/user")
        .service(link_entry::register())
        .service(list::register())
        .service(search::register())
}
