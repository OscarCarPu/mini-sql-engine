use crate::core::DataType;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Select,
    Star,
    Create,
    Open,
    Close,
    Comma,
    DotComma,
    Type(DataType),
    Name(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match self {
            Token::Select => "SELECT",
            Token::Star => "*",
            Token::Create => "CREATE",
            Token::Open => "(",
            Token::Close => ")",
            Token::Comma => ",",
            Token::DotComma => ";",
            Token::Type(data_type) => return write!(f, "{}", data_type),
            Token::Name(n) => n,
        };
        write!(f, "{}", t)
    }
}

pub fn tokenize(query: &str) -> Result<Vec<Token>, String> {
    let spaced = query
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace(',', " , ")
        .replace(';', " ; ");
    let mut tokens: Vec<Token> = Vec::new();
    for word in spaced.split_whitespace() {
        match word.to_uppercase().as_str() {
            "SELECT" => tokens.push(Token::Select),
            "*" => tokens.push(Token::Star),
            "CREATE" => tokens.push(Token::Create),
            "INT" => tokens.push(Token::Type(DataType::Int)),
            "FLOAT" => tokens.push(Token::Type(DataType::Float)),
            "TEXT" => tokens.push(Token::Type(DataType::Text)),
            "BOOL" => tokens.push(Token::Type(DataType::Bool)),
            "(" => tokens.push(Token::Open),
            ")" => tokens.push(Token::Close),
            "," => tokens.push(Token::Comma),
            ";" => tokens.push(Token::DotComma),
            _ => tokens.push(Token::Name(word.to_string())),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_select_star() {
        let result = tokenize("SELECT *");

        assert_eq!(result, Ok(vec![Token::Select, Token::Star]));
    }

    #[test]
    fn tokenize_create() {
        let result = tokenize("CREATE USERS ( id INT name TEXT )");

        assert_eq!(
            result,
            Ok(vec![
                Token::Create,
                Token::Name("USERS".to_string()),
                Token::Open,
                Token::Name("id".to_string()),
                Token::Type(DataType::Int),
                Token::Name("name".to_string()),
                Token::Type(DataType::Text),
                Token::Close,
            ])
        );
    }

    #[test]
    fn tokenize_create_unspaced() {
        let result = tokenize("CREATE USERS (id INT, name TEXT);");

        assert_eq!(
            result,
            Ok(vec![
                Token::Create,
                Token::Name("USERS".to_string()),
                Token::Open,
                Token::Name("id".to_string()),
                Token::Type(DataType::Int),
                Token::Comma,
                Token::Name("name".to_string()),
                Token::Type(DataType::Text),
                Token::Close,
                Token::DotComma
            ])
        )
    }
}
