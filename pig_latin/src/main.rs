fn pig_latin(list_vowels: Vec<char>, text: &str) -> String {
    let mut pig_latin = String::new();
    for word in text.split_whitespace() {
        let first_char = &word.chars().next().unwrap();
        let mut chars = word.chars();
        chars.next();
        let chars = chars.as_str();
        
        if list_vowels.iter().any(|i| i == first_char) {
            let pig = format!("{word}-hay ");
            pig_latin.push_str(&pig);
        } else {
            let pig = format!("{chars}-{first_char}ay ");
            pig_latin.push_str(&pig);
        }
    }
    pig_latin.trim_end().to_string()
}
fn main() {
    let list_vowels = vec!['a', 'e', 'i', 'u', 'y', 'o'];
    let text = "have you ever been hated or discriminated against";

    println!("{}", pig_latin(list_vowels, &text));
}
