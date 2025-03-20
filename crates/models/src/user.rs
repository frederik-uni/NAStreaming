use surrealdb::Datetime;

pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub updated: Datetime,
    pub created: Datetime,
}
