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
        assert!(table.insert(vec![Value::Int(1)]).is_ok());

        let mut database = Database {
            tables: vec![table],
        };

        let Ok(output) = execute("select *", &mut database) else {
            panic!("Got error on select ");
        };

        assert_eq!(output, "id(Int)\n1\n");
    }

    #[test]
    fn create_then_select_query() {
        let mut database = Database { tables: vec![] };

        let Ok(created) = execute("CREATE PETS ( id INT name TEXT )", &mut database) else {
            panic!("Got error on create pets");
        };
        let Ok(selected) = execute("select *", &mut database) else {
            panic!("Got error on select")
        };

        assert_eq!(created, "table 'PETS' created");
        assert_eq!(selected, "id(Int) | name(Text)\n");
    }
}
