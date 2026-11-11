use crate::tokenizer::Token;
use crate::{Column, SqlError};

#[derive(Debug, PartialEq)]
pub enum Statement {
    SelectAll,
    CreateTable { name: String, columns: Vec<Column> },
}

pub fn parse(tokens: &[Token]) -> Result<Statement, SqlError> {
    let tokens = match tokens {
        [rest @ .., Token::Semicolon] => rest,
        _ => tokens,
    };
    match tokens {
        [Token::Select, Token::Star] => Ok(Statement::SelectAll),
        [Token::Create, rest @ ..] => parse_create(rest),
        _ => Err(SqlError::UnsupportedQuery),
    }
}

fn parse_create(tokens: &[Token]) -> Result<Statement, SqlError> {
    let [name, open, defs @ .., close] = tokens else {
        return Err(SqlError::UnexpectedEnd);
    };
    let Token::Name(name) = name else {
        return Err(SqlError::ReservedName(name.to_string()));
    };
    if *open != Token::Open {
        return Err(SqlError::ExpectedOpenParam {
            found: open.to_string(),
        });
    }
    if *close != Token::Close {
        return Err(SqlError::ExpectedCloseParam {
            found: close.to_string(),
        });
    }
    let columns = defs
        .split(|token| *token == Token::Comma)
        .map(parse_column)
        .collect::<Result<Vec<Column>, SqlError>>()?;
    Ok(Statement::CreateTable {
        name: name.clone(),
        columns,
    })
}

fn parse_column(tokens: &[Token]) -> Result<Column, SqlError> {
    match tokens {
        [Token::Name(name), Token::Type(data_type)] => Ok(Column {
            name: name.clone(),
            data_type: data_type.clone(),
        }),
        _ => Err(SqlError::InvalidColumnDefinition),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DataType;
    use crate::tokenizer::tokenize;

    fn parse_str(sql: &str) -> Result<Statement, SqlError> {
        parse(&tokenize(sql)?)
    }

    #[test]
    fn select_all() {
        assert_eq!(parse_str("SELECT *"), Ok(Statement::SelectAll));
        assert_eq!(parse_str("select *;"), Ok(Statement::SelectAll));
    }

    #[test]
    fn create_table() {
        assert_eq!(
            parse_str("CREATE users (id INT, name TEXT);"),
            Ok(Statement::CreateTable {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: DataType::Int
                    },
                    Column {
                        name: "name".to_string(),
                        data_type: DataType::Text
                    },
                ],
            })
        );
    }
    #[test]
    fn unsupported() {
        assert_eq!(parse_str("* *"), Err(SqlError::UnsupportedQuery));
    }

    #[test]
    fn create_missing_open() {
        assert_eq!(
            parse_str("CREATE users id INT)"),
            Err(SqlError::ExpectedOpenParam {
                found: "id".to_string()
            })
        );
    }

    #[test]
    fn create_invalid_column() {
        assert_eq!(
            parse_str("CREATE users (id INT name TEXT)"),
            Err(SqlError::InvalidColumnDefinition)
        );
        assert_eq!(
            parse_str("CREATE users ()"),
            Err(SqlError::InvalidColumnDefinition)
        );
    }
}
