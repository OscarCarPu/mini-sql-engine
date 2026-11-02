use std::fmt;

use crate::core::DataType;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlError {
    UnsupportedQuery,
    NoTables,
    UnexpectedEnd,
    ReservedName(String),
    ExpectedOpenParam {
        found: String,
    },
    ExpectedCloseParam {
        found: String,
    },
    ColumnCountMismatch {
        expected: usize,
        got: usize,
    },
    TypeMismatch {
        column: String,
        expected: DataType,
        got: DataType,
    },
    InvalidColumnDefinition,
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SqlError::UnsupportedQuery => write!(f, "unsupported query"),
            SqlError::NoTables => write!(f, "there isn't any tables"),
            SqlError::UnexpectedEnd => write!(f, "unexpected end of query"),
            SqlError::ReservedName(name) => {
                write!(f, "reserved key '{}' used for table name", name)
            }
            SqlError::ExpectedOpenParam { found } => {
                write!(f, "Expected (, got '{}'", found)
            }
            SqlError::ExpectedCloseParam { found } => {
                write!(f, "Expected ), got '{}'", found)
            }
            SqlError::ColumnCountMismatch { expected, got } => {
                write!(f, "expected {} columns, got {}", expected, got)
            }
            SqlError::TypeMismatch {
                column,
                expected,
                got,
            } => {
                write!(
                    f,
                    "on '{}' expected data type {}, got {}",
                    column, expected, got
                )
            }
            SqlError::InvalidColumnDefinition => write!(f, "invalid column definition"),
        }
    }
}

impl std::error::Error for SqlError {}
