use surrealdb::Datetime;

use crate::{table, utils::DbUtils};

table!(User, "users");
pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub updated: Datetime,
    pub created: Datetime,
}

#[cfg(test)]
mod tests {
    use crate::connect;

    use super::*;

    #[tokio::test]
    async fn test_empty() {
        connect().await.unwrap();
        assert_eq!(User::empty().await.unwrap(), false);
    }
}
