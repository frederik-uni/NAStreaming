pub mod episodes;
pub mod file_group;
pub mod files;
pub mod lists;
pub mod metadata;
pub mod progress;
pub mod scan_groups;
pub mod user;
mod utils;

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
pub use surrealdb::Error;
use surrealdb::{RecordId, Surreal};
pub use utils::DbUtils;

#[derive(Deserialize, Serialize)]
pub struct Record<T> {
    pub id: RecordId,
    pub data: T,
}

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
pub async fn connect() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("nastreaming").use_db("nastreaming").await?;
    Ok(())
}
