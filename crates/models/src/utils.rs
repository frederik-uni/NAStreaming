use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::{Error, RecordId};

use crate::{Record, DB};

#[derive(Deserialize, Serialize)]
pub struct RecordIdTyped<T: DeserializeOwned> {
    id: RecordId,
    #[serde(skip)]
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> From<RecordId> for RecordIdTyped<T> {
    fn from(value: RecordId) -> Self {
        RecordIdTyped::new(value)
    }
}

impl<T: DeserializeOwned> Clone for RecordIdTyped<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned> RecordIdTyped<T> {
    pub fn id(&self) -> &RecordId {
        &self.id
    }
    pub fn get(self) -> impl std::future::Future<Output = Result<Option<Record<T>>, Error>> {
        async move { DB.select(self.id).await }
    }
    pub fn new(id: RecordId) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait DbUtils: DeserializeOwned + Serialize + 'static {
    fn table() -> &'static str;
    fn to_id(key: &str) -> RecordIdTyped<Self> {
        RecordIdTyped::from(RecordId::from((Self::table(), key)))
    }
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

    fn add_bulk(
        entries: Vec<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Record<Self>>, Error>> {
        async {
            let data = DB.insert(Self::table()).content(entries).await?;
            Ok(data)
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

#[derive(Deserialize, Serialize)]
pub struct Empty {
    pub id: RecordId,
}
