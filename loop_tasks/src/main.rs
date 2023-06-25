use std::io;

fn main() {
    println!("Enter Fahrenheit degree:");
    let mut f_degree = String::new();
    io::stdin()
        .read_line(&mut f_degree)
        .expect("Wrong enter!");
    
    let f_degree: i32 = f_degree.trim().parse().unwrap();
    
    let c_degree = (f_degree - 32) * 5/9;
    println!("In Celsius it is: {c_degree}");
}
