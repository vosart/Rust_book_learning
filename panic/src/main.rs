//use std::fs::File;
//use std::io::ErrorKind;
//use std::io::{self, Read};
use std::fs;
use std::io;

fn main() {

    // // with match
    
    //let greet_file_result = File::open("hello.txt");
    // let greet_file = match greet_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {:?}", e),
                
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error);
    //         }
    //     },
    // };
    
    // with closure
    // let greet_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Couldn't create the file {:?}", error);
    //         })

    //     } else {
    //         panic!("Couldn't open the file {:?}", error);
    //     }
    // });

    // проброс ошибок - функция, возвращающая ошибки в вызывающий код с помощью оператора ?
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }
    // let username = read_username_from_file();
    // println!("{:?}", username);

    // Использование fs::read_to_string вместо открытия и последующего чтения файла
    
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let username = read_username_from_file();
    println!("{:?}", username.unwrap());
}
