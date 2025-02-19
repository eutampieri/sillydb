mod sqlite;
pub trait Database {
    type Error: std::fmt::Debug;
    type Row;
    fn execute(&self, query: &str) -> Result<(), Self::Error>;
    fn query(
        &self,
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
