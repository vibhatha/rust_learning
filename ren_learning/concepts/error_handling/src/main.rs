use std::fs::File;

fn main() {
    println!("Hello, world!"); 
   
}

fn show_error() {
 let cargo_file_result = File::open("Cargo1.toml");               
                                                                  
 // This is going to panic.                                       
 let cargo_file = match cargo_file_result {                       
     Ok(file) => file,                                            
     Err(error) => panic!("Problem opening the file: {error:?}"), 
 };
} 
