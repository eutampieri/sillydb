use postgres::fallible_iterator::FallibleIterator;

use super::Database;
mod utils;

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
        let query = utils::map_query(query);
        let statement = self.prepare(&query)?;
        let params = params
            .into_iter()
            .zip(statement.params())
            .map(|(x, t)| utils::value_generic_to_concrete(x, t));
        Ok(self
            .query_raw(&statement, params)?
            .iterator()
            .filter_map(|x| x.ok())
            .map(utils::row_conversion)
            .collect())
    }
}
