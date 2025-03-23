mod dispatch;
mod list;
mod state;

use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/services")
        .service(dispatch::register())
        .service(list::register())
        .service(state::register())
}
