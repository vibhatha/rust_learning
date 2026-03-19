fn main() {
    println!("Common Programming Concepts!");

    let mut x=5;

    println!("The value of x is: {x}");

    x=6;

    println!("The value of x is: {x}");

    const A_COEFFICIENT:i32 = 2;
    const B_COEFFICIENT:f32 = 3.14;

    println!("A_COEFFICIENT is {A_COEFFICIENT}");
    println!("B_COEFFICIENT is {B_COEFFICIENT}");

    {
        let x = x + 2;
        println!("The value x in a scope: {x}")
    }

    println!("The value x outside the scope: {x}");

    // Data Types

    let guess: f32 = "42.431".parse().expect("Not a number");
    println!("The guess is {guess}");

    let number: u8 = 255;
    println!("The number is {number}");

    some_function();

}

fn some_function() {
    println!("This is some function");
}


