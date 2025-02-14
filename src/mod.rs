mod persister;
#[cfg(test)]
mod sqlite;

pub use persister::{DatabasePersister, DatabasePersisterFactory};
use serde::Deserialize;

pub trait Database {
    type Error;
    type Row;
    fn execute(&self, query: &str) -> Result<(), Self::Error>;
    fn query(&self, query: &str, params: &[SqlValue]);
    fn parse_row<D: for<'a> Deserialize<'a>>(row: Self::Row) -> D;
}

pub enum SqlValue {
    String(String),
    Unsigned64(u64),
}
