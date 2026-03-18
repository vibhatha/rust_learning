fn main() {
    println!("Hello, world!");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let y: u8 = 250;
    let q: u16 = y as u16 + 10; // casting

    println!("The value of q is: {q}");h

    let expr = {
        let x = 3;
        x + 1
    };

    println!("The value of expr is: {expr}");
}
