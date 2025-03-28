use apistos::web::Scope;

mod add;
mod delete;
mod delete_full;
mod edit;
mod info;
mod search;

pub fn register() -> apistos::web::Scope {
    Scope::new("/entry")
        .service(add::register())
        .service(delete::register())
        .service(delete_full::register())
        .service(edit::register())
        .service(info::register())
        .service(search::register())
}
