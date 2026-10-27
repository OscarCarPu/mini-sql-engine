use crate::core::{Column, Table, Value};
use crate::tokenizer::Token;
use std::fmt;

pub enum QueryResult {
    Rows(Vec<Column>, Vec<Vec<Value>>),
    Created(Table),
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryResult::Rows(columns, rows) => {
                let header: Vec<String> = columns.iter().map(|c| c.to_string()).collect();
                writeln!(f, "{}", header.join(" | "))?;
                for row in rows {
                    let cells: Vec<String> = row.iter().map(|v| v.to_string()).collect();
                    writeln!(f, "{}", cells.join(" | "))?;
                }
                Ok(())
            }
            QueryResult::Created(table) => write!(f, "table '{}' created", table.name),
        }
    }
}

#[derive(Default)]
pub struct Database {
    pub tables: Vec<Table>,
}
impl Database {
    pub fn run(&mut self, tokens: Vec<Token>) -> Result<QueryResult, String> {
        match tokens.as_slice() {
            [Token::Select, Token::Star, ..] => self.select_all(),
            [Token::Create, rest @ ..] => self.create(rest),
            _ => Err("unsopported query".to_string()),
        }
    }

    fn select_all(&self) -> Result<QueryResult, String> {
        let table = self.tables.first().ok_or("there isn't any tables")?;
        let (columns, rows) = table.select_all();
        Ok(QueryResult::Rows(columns.clone(), rows.clone()))
    }

    fn create(&mut self, tokens: &[Token]) -> Result<QueryResult, String> {
        let [name, open, rest @ ..] = tokens else {
            return Err("unexpected end of query".to_string());
        };
        let Token::Name(table_name) = name else {
            return Err(format!("reserved key '{}' used for table name", name));
        };
        if *open != Token::Open {
            return Err(format!("Expected '(' after table name, got '{}'", open));
        }
        let Some((close, defs)) = rest.split_last() else {
            return Err("unexpected end of query".to_string());
        };
        if *close != Token::Close {
            return Err(format!(
                "Expected ')' after column definition, got {}",
                close
            ));
        }
        let columns = defs
            .chunks(2)
            .map(|col| match col {
                [Token::Name(name), Token::Type(data_type)] => Ok(Column {
                    name: name.clone(),
                    data_type: data_type.clone(),
                }),
                _ => Err("invalid column definition".to_string()),
            })
            .collect::<Result<Vec<Column>, String>>()?;

        let table = Table::create(table_name.clone(), columns);
        self.tables.push(table.clone());
        Ok(QueryResult::Created(table))
    }
}

#[cfg(test)]
#[allow(clippy::indexing_slicing)]
mod tests {
    use super::*;
    use crate::core::DataType;

    fn users_table() -> Table {
        let mut table = Table::create(
            "users".to_string(),
            vec![
                Column {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                Column {
                    name: "name".to_string(),
                    data_type: DataType::Text,
                },
            ],
        );
        assert!(
            table
                .insert(vec![Value::Int(1), Value::Text("Ana".to_string())])
                .is_ok()
        );
        assert!(
            table
                .insert(vec![Value::Int(2), Value::Text("Pedro".to_string())])
                .is_ok()
        );
        table
    }

    #[test]
    fn run() {
        let mut database = Database {
            tables: vec![users_table()],
        };
        let Ok(QueryResult::Rows(columns, rows)) = database.run(vec![Token::Select, Token::Star])
        else {
            panic!("expected rows");
        };

        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].name, "id");
        assert_eq!(columns[1].name, "name");

        assert_eq!(
            rows,
            vec![
                vec![Value::Int(1), Value::Text("Ana".to_string())],
                vec![Value::Int(2), Value::Text("Pedro".to_string())]
            ]
        );
    }

    #[test]
    fn run_create() {
        let mut database = Database { tables: vec![] };
        let tokens = vec![
            Token::Create,
            Token::Name("pets".to_string()),
            Token::Open,
            Token::Name("id".to_string()),
            Token::Type(DataType::Int),
            Token::Name("name".to_string()),
            Token::Type(DataType::Text),
            Token::Close,
        ];

        let Ok(QueryResult::Created(table)) = database.run(tokens) else {
            panic!("expected created table");
        };

        assert_eq!(table.name, "pets");
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.columns[0].name, "id");
        assert_eq!(table.columns[1].data_type, DataType::Text);
        assert_eq!(database.tables.len(), 1);
    }

    #[test]
    fn run_unsupported() {
        let mut database = Database {
            tables: vec![users_table()],
        };
        let result = database.run(vec![Token::Star, Token::Star]);
        assert!(result.is_err());
    }
}
