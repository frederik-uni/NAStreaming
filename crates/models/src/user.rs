use serde::{Deserialize, Serialize};
pub use surrealdb::Datetime;

use crate::{table, Record};

table!(User, "users");
#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub role: Role,
    pub birthdate: Datetime,
    pub icon: Option<String>,
    pub password_hash: String,
    pub updated: Datetime,
    pub created: Datetime,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Role {
    Admin,
    User,
    None,
}

impl User {
    pub async fn find(username: &str) -> surrealdb::Result<Record<Self>> {
        todo!()
    }
    pub async fn has_email(email: &str) -> surrealdb::Result<bool> {
        todo!()
    }
    pub async fn has_name(email: &str) -> surrealdb::Result<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{connect, DbUtils as _};

    use super::*;

    #[tokio::test]
    async fn test_empty() {
        connect().await.unwrap();
        assert_eq!(User::empty().await.unwrap(), false);
    }
}
