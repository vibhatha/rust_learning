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

    complex_loop(5);

    print_triangle_with_loop(5);
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

fn complex_loop(height: i32) {
    let mut outer_loop_counter: i32 = 0;
    'outer_loop: loop {
        println!("Outer loop counter: {outer_loop_counter}");
        let mut inner_loop_counter: i32 = 0;
        'inner_loop: loop {
            println!("\tInner loop counter: {inner_loop_counter}");
            inner_loop_counter += 1;
            if inner_loop_counter == height {
                break 'inner_loop;
            }
        }
        outer_loop_counter += 1;
        if outer_loop_counter == height {
            break 'outer_loop;
        }
    }
}

fn print_triangle_with_loop(height: i32) {
    let mut i: i32 = 0;
    'outer_loop: loop {
        let mut j: i32 = 0;
        'inner_loop: loop {
            if i >= j {
                print!("*");
            } else {
                break 'inner_loop;
            }
            j += 1;
        }
        println!();
        i += 1;
        if i == height {
            break 'outer_loop;
        }
    }
}
