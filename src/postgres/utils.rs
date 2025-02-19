use crate::SqlValue;

#[derive(Debug)]
struct Ctx {
    idx: usize,
    result: String,
    region: Option<char>,
}

impl Ctx {
    fn append(self, c: String) -> Self {
        Self {
            idx: self.idx,
            result: self.result + &c,
            region: self.region,
        }
    }
    fn inc(self) -> Self {
        Self {
            idx: self.idx + 1,
            result: self.result,
            region: self.region,
        }
    }
    fn start(self, region: char) -> Self {
        if self.region.is_none() {
            Self {
                idx: self.idx,
                result: self.result + &region.to_string(),
                region: Some(region),
            }
        } else {
            self.append(region.to_string())
        }
    }
    fn end(self, region: char) -> Self {
        if let Some(r) = self.region {
            if r == region {
                Self {
                    idx: self.idx,
                    result: self.result + &r.to_string(),
                    region: None,
                }
            } else {
                self.append(region.to_string())
            }
        } else {
            self.append(region.to_string())
        }
    }
}

pub fn map_query(q: &str) -> String {
    q.chars()
        .into_iter()
        .fold(
            Ctx {
                idx: 0,
                result: "".to_owned(),
                region: None,
            },
            |acc, x| match x {
                '?' if acc.region.is_none() => {
                    let a = acc.inc();
                    let idx = a.idx;
                    a.append(format!("${}", idx))
                }
                '"' | '`' | '\'' => {
                    if acc.region.is_none() {
                        acc.start(x)
                    } else {
                        acc.end(x)
                    }
                }
                _ => acc.append(x.to_string()),
            },
        )
        .result
}

pub fn value_generic_to_concrete(v: &crate::SqlValue) -> Box<dyn postgres::types::ToSql + Sync> {
    match v {
        crate::SqlValue::String(v) => Box::new(v.clone()),
        crate::SqlValue::Integer(v) => Box::new(*v),
        crate::SqlValue::Binary(v) => Box::new(v.clone()),
        crate::SqlValue::Float(v) => Box::new(*v),
        crate::SqlValue::Null => Box::<Option<i8>>::new(None),
    }
}

pub fn row_conversion(r: postgres::Row) -> std::collections::HashMap<String, crate::SqlValue> {
    use postgres::types::Type;
    r.columns()
        .into_iter()
        .map(|x| (x.name(), x.type_()))
        .map(|(n, t)| {
            (
                n,
                match t {
                    &Type::BOOL => r
                        .get::<_, Option<bool>>(n)
                        .map(|v| SqlValue::Integer(if v { 1 } else { 0 })),
                    &Type::TEXT | &Type::CHAR_ARRAY | &Type::VARCHAR | &Type::CHAR => {
                        r.get::<_, Option<String>>(n).map(|x| SqlValue::String(x))
                    }
                    &Type::INT2 | &Type::INT4 | &Type::INT8 => {
                        r.get::<_, Option<i64>>(n).map(|x| SqlValue::Integer(x))
                    }
                    &Type::BYTEA => r.get::<_, Option<Vec<u8>>>(n).map(|x| SqlValue::Binary(x)),
                    &Type::FLOAT4 | &Type::FLOAT8 => {
                        r.get::<_, Option<f64>>(n).map(|x| SqlValue::Float(x))
                    }
                    _ => unimplemented!(),
                },
            )
        })
        .map(|(k, v)| {
            (
                k.to_string(),
                if let Some(x) = v { x } else { SqlValue::Null },
            )
        })
        .collect()
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

    #[test]
    fn query_conversion_doesnt_change_question_marks_in_str() {
        let query = "SELECT * FROM test WHERE a = '?';";
        let actual = map_query(query);
        assert_eq!(query, actual);
    }

    #[test]
    fn query_conversion_doesnt_change_question_marks_in_str_double() {
        let query = "SELECT * FROM test WHERE a = \"?\";";
        let actual = map_query(query);
        assert_eq!(query, actual);
    }

    #[test]
    fn query_conversion_doesnt_change_question_marks_in_backticks() {
        let query = "SELECT * FROM test WHERE a = `?`;";
        let actual = map_query(query);
        assert_eq!(query, actual);
    }
}
