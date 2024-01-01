mod table;

use table::Array;
use table::Column;
use table::DataType;
use table::Table;

fn create_table() {
    let array1 = Array::new(DataType::Int(vec![1, 2, 3]));
    let column1 = Column::new(String::from("C1"), array1);

    let array2 = Array::new(DataType::Float(vec![1.1, 2.1, 3.1]));
    let column2 = Column::new(String::from("C2"), array2);

    let array3 = Array::new(DataType::Text(vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
    ]));
    let column3 = Column::new(String::from("C3"), array3);

    let table = Table::new(vec![column1, column2, column3]);
    println!("{}", table.to_string());
}

fn main() {
    println!("Hello, world!");
    create_table();
}
