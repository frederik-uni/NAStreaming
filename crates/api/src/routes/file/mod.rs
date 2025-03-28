mod get_dir;
mod link_entry;
mod list_unlinked;
mod overview_unlinked;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/file")
        .service(link_entry::register())
        .service(overview_unlinked::register())
        .service(list_unlinked::register())
        .service(get_dir::register())
}
