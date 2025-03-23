use serde::{Deserialize, Serialize};
pub use surrealdb::Datetime;

use crate::{table, Record, DB};

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
    pub async fn find(username: &str) -> surrealdb::Result<Option<Record<Self>>> {
        let v: Option<Record<Self>> = DB
            .query("SELECT * FROM users WHERE name = $user LIMIT 1")
            .bind(("user", username.to_string()))
            .await?
            .take(0)?;
        Ok(v)
    }
    pub async fn has_email(email: &str) -> surrealdb::Result<bool> {
        let v: Option<bool> = DB
            .query("(SELECT count() FROM users WHERE email = $email LIMIT 1) == 1")
            .bind(("email", email.to_string()))
            .await?
            .take(0)?;
        Ok(v.unwrap_or_default())
    }
    pub async fn has_name(user: &str) -> surrealdb::Result<bool> {
        let v: Option<bool> = DB
            .query("(SELECT count() FROM users WHERE name = $user LIMIT 1) == 1")
            .bind(("user", user.to_string()))
            .await?
            .take(0)?;
        Ok(v.unwrap_or_default())
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
