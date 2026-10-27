// src/lib.rs
mod core;
mod run;
mod tokenizer;

pub use crate::core::{Column, DataType, Table, Value};
pub use crate::run::Database;
pub use crate::tokenizer::tokenize;

pub fn execute(sql: &str, database: &mut Database) -> Result<String, String> {
    let tokens = tokenize(sql)?;
    let result = database.run(tokens)?;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_all_query() {
        let mut table = Table::create(
            "users".to_string(),
            vec![Column {
                name: "id".to_string(),
                data_type: DataType::Int,
            }],
        );
        table.insert(vec![Value::Int(1)]).unwrap();

        let mut database = Database {
            tables: vec![table],
        };

        let output = execute("select *", &mut database).unwrap();

        assert_eq!(output, "id(Int)\n1\n");
    }

    #[test]
    fn create_then_select_query() {
        let mut database = Database { tables: vec![] };

        let created = execute("CREATE PETS ( id INT name TEXT )", &mut database).unwrap();
        let selected = execute("select *", &mut database).unwrap();

        assert_eq!(created, "table 'PETS' created");
        assert_eq!(selected, "id(Int) | name(Text)\n");
    }
}
