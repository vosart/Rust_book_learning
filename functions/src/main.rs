use std::io;
fn main() {
    println!("Hello, world!");
    another_function();
    echo_function(6);
    print_labeled_measurment(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
    
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(num) => num,
        Err(_) => panic!("Something wrong!"),
    };
}

fn another_function() {
    println!("Another awesome function!");
}

fn echo_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurment(value: i32, unit_label: char) {
    println!("The measurment is: {value}{unit_label}");
}

fn five() -> i32 {
//!!! чтобы возратить значение нужно не ставить ";"  !!!
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}