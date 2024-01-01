use std::fmt;

pub enum DataType {
    Int(Vec<i32>),
    Float(Vec<f64>),
    Text(Vec<String>),
}

pub struct Array {
    data: DataType,
}

impl Array {
    pub fn new(data: DataType) -> Self {
        Self { data }
    }
}

pub struct Column {
    name: String,
    array: Array,
}

impl Column {
    pub fn new(name: String, array: Array) -> Self {
        Self { name, array }
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
            match &column.array.data {
                DataType::Int(values) => {
                    for (index, value) in values.iter().enumerate() {
                        write!(f, "{}", value)?;
                        if index < values.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                }
                DataType::Float(values) => {
                    for (index, value) in values.iter().enumerate() {
                        write!(f, "{}", value)?;
                        if index < values.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                }
                DataType::Text(values) => {
                    for (index, value) in values.iter().enumerate() {
                        write!(f, "{}", value)?;
                        if index < values.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
