use std::sync::{Arc, Mutex};

use actix_web::web::Data;
use apistos::web::Scope;
use models::{user, DbUtils as _};

use crate::{
    config::Config,
    services::{auth::AuthService, metadata::MetadataService, Services},
};

pub struct UserExists {
    pub exists: bool,
}

impl UserExists {
    pub async fn init() -> Self {
        let v = user::User::empty()
            .await
            .expect("failed to check if any users exit");
        UserExists { exists: !v }
    }
}

pub fn app_data_scope(config: Arc<Config>, user_exists: Arc<Mutex<UserExists>>) -> Scope {
    Scope::new("/api")
        .app_data(Data::from(user_exists))
        .app_data(Data::from(Services::new()))
        .app_data(Data::new(
            MetadataService::new(config.others.clone()).expect("Failed to init metadata service"),
        ))
        .app_data(Data::new(AuthService::new(
            config.server.secret_key.as_bytes().to_vec(),
        )))
        .app_data(Data::from(config))
}
