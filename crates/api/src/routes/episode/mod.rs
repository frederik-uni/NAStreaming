use apistos::web::Scope;

mod add;
mod delete;
mod delete_full;
mod info;

pub fn register() -> apistos::web::Scope {
    Scope::new("/file")
        .service(add::register())
        .service(delete::register())
        .service(delete_full::register())
        .service(info::register())
}
