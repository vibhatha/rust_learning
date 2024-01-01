use std::fmt;

pub enum DataType {
    Int(i32),
    Float(f64),
    Text(String),
}

pub struct Column {
    name: String,
    data: Vec<DataType>,
}

impl Column {
    pub fn new(name: String, data: Vec<DataType>) -> Self {
        Self { name, data }
    }
}

pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self { columns }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for column in &self.columns {
            write!(f, "{}: ", column.name)?;
            for (index, data) in column.data.iter().enumerate() {
                match data {
                    DataType::Int(value) => write!(f, "{}", value)?,
                    DataType::Float(value) => write!(f, "{}", value)?,
                    DataType::Text(value) => write!(f, "{}", value)?,
                }
                if index < column.data.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
