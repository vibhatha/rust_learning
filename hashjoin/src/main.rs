mod table;

use table::Column;
use table::DataType;
use table::Table;

fn create_table() {
    let column1 = Column::new(
        String::from("Column 1"),
        vec![DataType::Int(1), DataType::Int(2), DataType::Int(3)],
    );

    let column2 = Column::new(
        String::from("Column 2"),
        vec![
            DataType::Text(String::from("Hello")),
            DataType::Text(String::from("World")),
        ],
    );

    let column3 = Column::new(
        String::from("Column 1"),
        vec![
            DataType::Float(1.1),
            DataType::Float(2.1),
            DataType::Float(3.1),
        ],
    );

    let table = Table::new(vec![column1, column2, column3]);
    println!("{}", table.to_string());
}

fn main() {
    println!("Hello, world!");
    create_table();
}
