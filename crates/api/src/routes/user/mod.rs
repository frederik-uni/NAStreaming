pub(crate) mod create;
mod edit;
mod info;
mod refresh;
mod sign_in;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/user")
        .service(create::register())
        .service(edit::register())
        .service(info::register())
        .service(refresh::register())
        .service(sign_in::register())
}
