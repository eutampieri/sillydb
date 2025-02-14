use super::Database;

mod deserializer;

impl Database for sqlite::Connection {
    type Error = sqlite::Error;
    type Row = sqlite::Row;

    fn execute(&self, query: &str) -> Result<(), Self::Error> {
        self.execute(query)
    }

    fn query(&self, query: &str, params: &[super::SqlValue]) {
        todo!()
    }

    fn parse_row<D: for<'a> serde::Deserialize<'a>>(row: Self::Row) -> D {
        D::deserialize(deserializer::SQLiteDeserializer(row)).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_db() -> impl Database {
        sqlite::open(":memory:").unwrap()
    }

    #[test]
    fn can_execute() {
        let db = get_db();
        assert!(db.execute("SELECT 1").is_ok());
    }
    #[test]
    fn execute_errors_on_invalid_sql() {
        let db = get_db();
        assert!(db.execute("broken sql").is_err());
    }
    #[test]
    fn can_query_no_params() {
        let db = get_db();
        let q = "SELECT 1 AS a";
        let params = &[];
        db.query(q, params);
    }
}
