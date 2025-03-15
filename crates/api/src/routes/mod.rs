use apistos::web::Scope;

pub fn register() -> apistos::web::Scope {
    Scope::new("/v1")
}
