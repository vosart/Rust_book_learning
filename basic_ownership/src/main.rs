fn main() {
    let s = String::from("hello");
    println!("The string is: {s}");
    takes_ownership(s);
    /*
    println!("The string is: {s}"); А так нельзя!
    */


    let x = 5;
    println!("The integer is: {x}");
    makes_copy(x);
    println!("The integer is: {x}");
   
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}