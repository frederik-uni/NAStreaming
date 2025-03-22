use surrealdb::Error;

use crate::DB;

pub trait DbUtils {
    fn table() -> &'static str;
    async fn empty() -> Result<bool, Error> {
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

#[macro_export]
macro_rules! table {
    ($struct_name:ident, $table_name:expr) => {
        impl DbUtils for $struct_name {
            fn table() -> &'static str {
                $table_name
            }
        }
    };
}
