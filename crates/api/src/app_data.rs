use std::sync::Arc;

use actix_web::web::Data;
use apistos::web::Scope;

use crate::config::Config;

pub fn app_data_scope(config: Arc<Config>) -> Scope {
    Scope::new("/").app_data(Data::from(config))
}
