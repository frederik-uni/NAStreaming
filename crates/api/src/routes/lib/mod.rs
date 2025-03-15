mod add;
mod delete;
mod edit;
mod list;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/lib")
        .service(add::register())
        .service(edit::register())
        .service(delete::register())
        .service(list::register())
}
