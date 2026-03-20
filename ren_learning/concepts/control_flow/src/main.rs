fn main() {
    println!("Hello, world!");
    basic_control_flow(5);
    basic_control_flow(11);
    let x = 5;
    let status: bool = basic_ternary_flow(x);
    println!("Basic Ternary flow x = {x} ==> {status}");
    let x = 11;
    let status: bool = basic_ternary_flow(x);
    println!("Basic Ternary flow x = {x} ==> {status}");

    let loop_result = basic_loop(3, 4);
    println!("Loop Result: {loop_result}");
}

fn basic_control_flow(x: i32) -> bool {
    let mut status: bool = false;
    if x < 10 {
        println!("The value {x} is in range");
        status = true;
    } else {
        println!("The value {x} is not in range");
    }
    status
}

fn basic_ternary_flow(x: i32) -> bool {
    let condition = if x < 10 {true} else {false};
    condition
}

fn basic_loop(break_value: i32, coeff: i32) -> i32 {
    println!("Break value: {break_value}, coeff: {coeff}");
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == break_value {
            break counter * coeff;
        }
    };
    result
}
