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
    another_function(32);

    let incr: i32 = incrementor(5);
    println!("Incremented value : {incr}");

}

fn some_function() {
    println!("This is some function");
}

fn another_function(x: i32) {
    println!("This is some function");
    println!("x value is: {x}");
}

fn incrementor(x: i32) -> i32 {
    let y: i32 = x + 1;
    y
}


