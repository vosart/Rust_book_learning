fn main() {
    let s = &String::from("Have you ever be hated or discriminated against?");
    let word = first_word(&s);
    println!("{word}");

    let a = [1, 2, 3, 4, 5, 6, 7];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];    
        }
    }
    
    &s[..] 
}