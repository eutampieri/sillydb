use super::Database;

fn map_query(q: &str) -> String {
    struct Ctx {
        idx: usize,
        result: String,
    }

    impl Ctx {
        fn append(self, c: String) -> Self {
            Self {
                idx: self.idx,
                result: self.result + &c,
            }
        }
        fn inc(self) -> Self {
            Self {
                idx: self.idx + 1,
                result: self.result,
            }
        }
    }

    q.chars()
        .into_iter()
        .fold(
            Ctx {
                idx: 0,
                result: "".to_owned(),
            },
            |acc, x| match x {
                '?' => {
                    let a = acc.inc();
                    let idx = a.idx;
                    a.append(format!("${}", idx))
                }
                _ => acc.append(x.to_string()),
            },
        )
        .result
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_conversion_changes_question_marks_to_dollar_index() {
        let query = "SELECT * FROM test WHERE a = ?;";
        let expected = "SELECT * FROM test WHERE a = $1;";
        let actual = map_query(query);
        assert_eq!(expected, actual);
    }
}
