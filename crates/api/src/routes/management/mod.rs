use apistos::web::Scope;

mod check_for_update;
mod restart;
mod stop;

pub fn register() -> apistos::web::Scope {
    Scope::new("/server-management")
        .service(check_for_update::register())
        .service(stop::register())
        .service(restart::register())
}
