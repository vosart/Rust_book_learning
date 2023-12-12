use std::fs::read_to_string;

fn line_to_digit(line: &String) -> i32 {
    
    let mut string_line= String::new();
    for char in line.chars() {
        if char.is_digit(10) {
            string_line.push(char);
        }
    }
    if string_line.len() == 1 {
        string_line = string_line.repeat(2);
    }
    let mut chars = string_line.chars();
    let first = chars.next();
    let last = chars.next_back();
    let mut result = String::new();
    result.push(first.unwrap());

    result.push(last.unwrap());
    result.parse::<i32>().unwrap()
}

fn main() {
    let mut calibration_value = 0;
    for line in read_to_string("text.txt").unwrap().lines() {
        calibration_value += line_to_digit(&line.to_string());
    }
    println!("{}", calibration_value);
}
