use super::Database;

impl Database for postgres::Client {
    type Error = postgres::Error;

    type Row = postgres::Row;

    fn execute(&mut self, query: &str) -> Result<(), Self::Error> {
        postgres::Client::execute(self, query, &[]).map(|_| ())
    }

    fn query(
        &mut self,
        query: &str,
        params: &[crate::SqlValue],
    ) -> Result<Vec<std::collections::HashMap<String, crate::SqlValue>>, Self::Error> {
        todo!()
    }
}
