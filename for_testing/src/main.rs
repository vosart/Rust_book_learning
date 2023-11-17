use std::{io, cmp::Ordering};


fn main() {
    let s1 = String::from("Let's get Rusty!");
    let s2 = String::from("LGR");

    let result: &str = longest(s1.as_str(), s2.as_str());

    println!("The longest string is {}", result);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
