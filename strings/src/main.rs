
// struct Person<'a> {
//     name: &'a str,
// }

// impl<'a> Person<'a> {
//     fn greet(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }
fn main() {
    // let name = String::from("Herman");
    // let person = Person { name: &name };
    // person.greet();
    // println!("{}", &name);

    // empty string
    // let mut s = String::new();

    // let data = "initial content";
    // let s = data.to_string();

    // // or
    // let p = "second init content".to_string();
    // println!("{s}\n{p}");
    
    // // or
    // let q = String::from("third content");
    // println!("{q}");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s1 is {s1}");

    // // push add 1 char to string
    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{s}");
    
    // // s1 move, &s2 - reference,
    // // "+" - fn add(self, &str) -> String {}
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // println!("{s3}");

    // // format!
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");
    // println!("{s}");

    // // итерация по строкам посимвольно
    // let s = String::from("Здравствуйте!");
    // for c in s.chars() {
    //     println!("{}", c);
    // }
    // итерация по строкам побайтно
    // let s = String::from("Здравствуйте!");
    // for c in s.bytes() {
    //     println!("{}", c);
    // }
    
    // получение n-го символа char
    // let s = String::from("Hello!");
    // let c = s.chars().nth(3).unwrap();
    // println!("{c}");

    
}
