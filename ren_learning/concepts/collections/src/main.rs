fn main() {
    println!("Hello Collections!");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("1 -> s2 is {s2}");
    println!("1 -> s3 is {s3}");

    s2 = String::from("Mundo!");

    println!("2 -> s2 is {s2}");
    println!("2 -> s3 is {s3}");
}
