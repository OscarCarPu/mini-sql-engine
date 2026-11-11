use crate::core::{Column, Table, Value};
use crate::error::SqlError;
use crate::parser::Statement;
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
    pub fn run(&mut self, statement: Statement) -> Result<QueryResult, SqlError> {
        match statement {
            Statement::SelectAll => self.select_all(),
            Statement::CreateTable { name, columns } => Ok(self.create_table(name, columns)),
        }
    }
    fn select_all(&self) -> Result<QueryResult, SqlError> {
        let table = self.tables.first().ok_or(SqlError::NoTables)?;
        let (columns, rows) = table.select_all();
        Ok(QueryResult::Rows(columns.clone(), rows.clone()))
    }

    fn create_table(&mut self, name: String, columns: Vec<Column>) -> QueryResult {
        let table = Table::create(name, columns);
        self.tables.push(table.clone());
        QueryResult::Created(table)
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
        let Ok(QueryResult::Rows(columns, rows)) = database.run(Statement::SelectAll) else {
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
        let statement = Statement::CreateTable {
            name: "pets".to_string(),
            columns: vec![
                Column {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                Column {
                    name: "name".to_string(),
                    data_type: DataType::Text,
                },
            ],
        };

        let Ok(QueryResult::Created(table)) = database.run(statement) else {
            panic!("expected created table");
        };

        assert_eq!(table.name, "pets");
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.columns[0].name, "id");
        assert_eq!(table.columns[1].data_type, DataType::Text);
        assert_eq!(database.tables.len(), 1);
    }

    #[test]
    fn run_select_no_tables() {
        let mut database = Database::default();
        let result = database.run(Statement::SelectAll);
        assert!(matches!(result, Err(SqlError::NoTables)));
    }
}
