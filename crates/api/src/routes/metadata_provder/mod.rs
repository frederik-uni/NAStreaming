mod providers;
mod search;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/metadata-provider")
        .service(search::register())
        .service(providers::register())
}
