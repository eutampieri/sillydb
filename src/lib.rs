#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;

pub mod db;

pub trait Database {
    type Error: std::fmt::Debug;
    type Row;
    fn execute(&mut self, query: &str) -> Result<(), Self::Error>;
    fn query(
        &mut self,
        query: &str,
        params: &[SqlValue],
    ) -> Result<Vec<std::collections::HashMap<String, SqlValue>>, Self::Error>;
}

#[derive(PartialEq, Debug)]
pub enum SqlValue {
    String(String),
    Integer(i64),
    Binary(Vec<u8>),
    Float(f64),
    Null,
}
