use std::any::type_name;
use std::rc::Rc;

fn main() {
    println!("Rust Ownership");
    let x = 5;
    let mut y = &x;

    println!("Before Mutation");
    println!("x : {}", x);
    println!("y : {}", y);

    y = &7;

    println!("After Mutation");
    println!("x : {}", x);
    println!("y : {}", y);

    let s1 = String::from("Hello");
    println!("S1 : {}", s1);
    let s1_len: usize = get_len(&s1);
    println!("S1 : {}, length: {}", s1, s1_len);

    let s4: String = String::from("");
    let s5: String;

    // make a scope and see how clone works
    {
        println!("In String Scope");
        let s2_placeholder = String::from("Hello World");
        s5 = s2_placeholder.clone();
        let s2 = Rc::new(s2_placeholder);
        let s3 = Rc::clone(&s2);
        let s4 = Rc::clone(&s2);

        println!("\tS2 : {}, type: {}", s2, get_type(&s2));
        println!("\tS3 : {}, type: {}", s3, get_type(&s3));
        println!("\tS4 : {}, type: {}", s4, get_type(&s4));
        println!("\tS5 : {}, type: {}", s5, get_type(&s5));
    }
    println!("Out of String Scope");
    println!("\tS4 : {}, type: {}", s4, get_type(&s4));
    println!("\tS5 : {}, type: {}", s5, get_type(&s5));
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn get_type<T>(_: &T) -> &'static str {
    type_name::<T>()
}