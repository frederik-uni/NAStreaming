use serde::{de::DeserializeOwned, Serialize};
use surrealdb::Error;

use crate::{Record, DB};

pub trait DbUtils: DeserializeOwned + Serialize + 'static {
    fn table() -> &'static str;
    fn empty() -> impl std::future::Future<Output = Result<bool, Error>> {
        async {
            Ok(DB
                .query(format!(
                    "array::len(SELECT count() from {} LIMIT 1) == 0",
                    Self::table()
                ))
                .await?
                .take::<Option<bool>>(0)?
                .unwrap_or_default())
        }
    }

    fn add(self) -> impl std::future::Future<Output = Result<Record<Self>, Error>> {
        async move {
            let mut data = DB.insert(Self::table()).content(vec![self]).await?;
            Ok(data.remove(0))
        }
    }

    fn get(s: &str) -> impl std::future::Future<Output = Result<Option<Record<Self>>, Error>> {
        let id = s.to_string();

        async move { DB.select((Self::table(), id.as_str())).await }
    }

    fn all() -> impl std::future::Future<Output = Result<Vec<Record<Self>>, Error>> {
        async move { DB.select(Self::table()).await }
    }
}

#[macro_export]
macro_rules! table {
    ($struct_name:ident, $table_name:expr) => {
        impl crate::utils::DbUtils for $struct_name {
            fn table() -> &'static str {
                $table_name
            }
        }
    };
}
