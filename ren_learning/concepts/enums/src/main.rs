fn main() {
    println!("Hello, world!");

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(4);

    if let Some(value) = y {
        let sum = x + value;
        println!("Sum : {}", sum);
    } else {
        println!("Could not calculate sum because y is None");
    }

    let sum = x + y.unwrap();
    println!("Sum : {}", sum);
    
    let messages = vec![
        Message::Quit,
        Message::Move {x: 10, y: 20},
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0)
    ];

    for msg in &messages {
        msg.process();
        msg.call();
    }

}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message::call");
    }

    fn process(&self) {
        match self {
            Message::Quit => {
                println!("Quit message");
            }
            Message::Move {x, y} => {
                println!("x = {}, y = {}", x, y);
            }
            Message::Write(text) => {
                println!("Text Message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to R:{}, G:{}, B:{}", r, g, b);
            }
        }
    }
}


