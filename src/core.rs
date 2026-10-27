use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum DataType {
    Int,
    Float,
    Text,
    Bool,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dt = match self {
            DataType::Int => "Int",
            DataType::Float => "Float",
            DataType::Text => "Text",
            DataType::Bool => "Bool",
        };
        write!(f, "{}", dt)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(x) => write!(f, "{}", x),
            Value::Text(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.data_type)
    }
}

#[derive(Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<Value>>,
}

impl Value {
    fn data_type(&self) -> Option<DataType> {
        match self {
            Value::Null => None,
            Value::Int(_) => Some(DataType::Int),
            Value::Float(_) => Some(DataType::Float),
            Value::Text(_) => Some(DataType::Text),
            Value::Bool(_) => Some(DataType::Bool),
        }
    }
}

impl Table {
    pub fn create(name: String, columns: Vec<Column>) -> Table {
        Table {
            name,
            columns,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: Vec<Value>) -> Result<(), String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "expected {} columns, got {}",
                self.columns.len(),
                row.len()
            ));
        }
        for (r, col) in row.iter().zip(self.columns.iter()) {
            if let Some(row_type) = r.data_type()
                && row_type != col.data_type
            {
                return Err(format!(
                    "on '{}' expected DataType {}, got {}",
                    col.name, col.data_type, row_type
                ));
            }
        }
        self.rows.push(row);
        Ok(())
    }

    pub fn select_all(&self) -> (&Vec<Column>, &Vec<Vec<Value>>) {
        (&self.columns, &self.rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn users_table() -> Table {
        let columns = vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Int,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::Text,
            },
        ];

        Table::create("users".to_string(), columns)
    }

    #[test]
    fn new_table() {
        let columns = vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Int,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::Text,
            },
        ];

        let table = Table::create("users".to_string(), columns.clone());
        assert_eq!(table.name, "users");
        assert_eq!(table.columns, columns);
    }

    #[test]
    fn insert() {
        let mut table = users_table();

        let result = table.insert(vec![Value::Int(1), Value::Text("Ana".to_string())]);

        assert_eq!(result, Ok(()));
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn insert_wrong_column_number() {
        let mut table = users_table();

        let result = table.insert(vec![Value::Int(1)]);

        assert_eq!(result, Err("expected 2 columns, got 1".to_string()));
        assert_eq!(table.rows.len(), 0);
    }

    #[test]
    fn insert_wrong_data_type() {
        let mut table = users_table();

        let result = table.insert(vec![
            Value::Text("1".to_string()),
            Value::Text("Ana".to_string()),
        ]);

        assert_eq!(
            result,
            Err("on 'id' expected DataType Int, got Text".to_string())
        );
        assert_eq!(table.rows.len(), 0);
    }
}
