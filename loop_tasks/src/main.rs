use std::io;

fn main() {
    println!("Enter Fahrenheit degree:");
    let mut f_degree = String::new();
    io::stdin()
        .read_line(&mut f_degree)
        .expect("Wrong number");
    
    let f_degree: i32 = match f_degree.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Wrong number"),
    };
    let c_degree = (f_degree - 32) * 5/9;
    println!("In Celsius it is: {c_degree}");
}
