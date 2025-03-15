mod create;
mod edit;
mod info;
mod sign_in;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/user")
        .service(create::register())
        .service(edit::register())
        .service(info::register())
        .service(sign_in::register())
}
