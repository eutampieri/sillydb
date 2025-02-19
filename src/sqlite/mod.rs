use super::Database;

fn convert_values(v: &super::SqlValue) -> sqlite::Value {
    match v {
        super::SqlValue::Integer(n) => sqlite::Value::Integer(*n),
        super::SqlValue::String(s) => sqlite::Value::String(s.clone()),
        super::SqlValue::Binary(x) => sqlite::Value::Binary(x.clone()),
        super::SqlValue::Float(x) => sqlite::Value::Float(*x),
        super::SqlValue::Null => sqlite::Value::Null,
    }
}

fn convert_native_to_abstracted_values(v: &sqlite::Value) -> super::SqlValue {
    match v {
        sqlite::Value::String(s) => super::SqlValue::String(s.clone()),
        sqlite::Value::Integer(x) => super::SqlValue::Integer(*x),
        sqlite::Value::Binary(x) => super::SqlValue::Binary(x.clone()),
        sqlite::Value::Float(x) => super::SqlValue::Float(*x),
        sqlite::Value::Null => super::SqlValue::Null,
    }
}

impl Database for sqlite::Connection {
    type Error = sqlite::Error;
    type Row = sqlite::Row;

    fn execute(&mut self, query: &str) -> Result<(), Self::Error> {
        sqlite::Connection::execute(&self, query)
    }

    fn query(
        &mut self,
        query: &str,
        params: &[super::SqlValue],
    ) -> Result<Vec<std::collections::HashMap<String, super::SqlValue>>, Self::Error> {
        let mut statement = self.prepare(query)?;
        statement.bind_iter(
            params
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1, convert_values(v))),
        )?;
        Ok(statement
            .iter()
            .filter_map(Result::ok)
            .map(|x| {
                x.iter()
                    .map(|(k, v)| (k.to_owned(), convert_native_to_abstracted_values(v)))
                    .collect()
            })
            .collect())
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
        let mut db = get_db();
        assert!(db.execute("SELECT 1").is_ok());
    }
    #[test]
    fn execute_errors_on_invalid_sql() {
        let mut db = get_db();
        assert!(db.execute("broken sql").is_err());
    }
    #[test]
    fn can_query_no_params() {
        let mut db = get_db();
        let q = "SELECT 1 AS a";
        let params = &[];
        let result = db.query(q, params);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(1, result.len());
        let row = &result[0];
        assert!(row.contains_key("a"));
        assert_eq!(&crate::SqlValue::Integer(1), row.get("a").unwrap())
    }
    #[test]
    fn can_query_with_params() {
        let mut db = get_db();
        db.execute("CREATE TABLE test(k TEXT, v INTEGER)").unwrap();
        db.execute("INSERT INTO test VALUES (\"a\", 1)").unwrap();
        db.execute("INSERT INTO test VALUES (\"b\", 2)").unwrap();
        let q = "SELECT * FROM test WHERE k = ?";
        let params = &[crate::SqlValue::String("a".to_owned())];
        let result = db.query(q, params);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(1, result.len());
        let row = &result[0];
        assert!(row.contains_key("v"));
        assert_eq!(&crate::SqlValue::Integer(1), row.get("v").unwrap())
    }
}
