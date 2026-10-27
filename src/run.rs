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
        if tokens[0] == Token::Select && tokens[1] == Token::Star {
            if self.tables.is_empty() {
                return Err("there isn't any tables".to_string());
            }
            let table = &self.tables[0];
            let result = table.select_all();
            return Ok(QueryResult::Rows(result.0.clone(), result.1.clone()));
        }
        if tokens[0] == Token::Create {
            let table_name = match &tokens[1] {
                Token::Name(name) => name,
                token_type => {
                    return Err(format!("reserved key '{}' used for table name", token_type));
                }
            };
            if tokens[2] != Token::Open {
                return Err(format!("Expected '(' after table name, got {}", tokens[2]));
            }
            let mut columns: Vec<Column> = Vec::new();
            for col in tokens[3..].split_last().unwrap().1.chunks(2) {
                match col {
                    [Token::Name(name), Token::Type(data_type)] => columns.push(Column {
                        name: name.clone(),
                        data_type: data_type.clone(),
                    }),
                    _ => return Err("invalid column definition".to_string()),
                }
            }
            if tokens[3 + columns.len() * 2] != Token::Close {
                return Err(format!(
                    "Expected ')' after column definition, got {}",
                    tokens[3 + columns.len() * 2]
                ));
            }
            let table = Table::create(table_name.clone(), columns);
            self.tables.push(table.clone());
            return Ok(QueryResult::Created(table));
        }
        Err("unsopported query".to_string())
    }
}

#[cfg(test)]
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
