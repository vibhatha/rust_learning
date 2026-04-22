fn main() {
    println!("Hello, world!");
    hello_rust();
    let mut z = add(10, 12);
    println!("The sum is {}", z);
    println!("Subtraction");
    z = subtract(12, 10);
    println!("The subtract return {}", z);
}

pub fn hello_rust() {
    println!("Hello This is from Rust");
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn subtract(x: i32, y:i32) -> i32 {
    return x - y;
} 
