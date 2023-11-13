use std::{io, cmp::Ordering};
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");
    
    let file_text = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem openning file {:?}", error),
        
    };
}
