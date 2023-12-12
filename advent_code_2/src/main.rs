use regex::Regex;
use std::fs::read_to_string;

fn id_sum(text: &str) -> i32 {
    let red_rule = 12;
    let green_rule = 13;
    let blue_rule = 14;
    let mut answer = 0;

    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re = Regex::new(r"\b\d{1,2} red\b|\b\d{1,2} green\b|\b\d{1,2} blue\b").unwrap();

    let game = re_game.captures(text).unwrap();
    let game_id = &game[1].parse::<i32>().unwrap();

    let sub: Vec<&str> = re.find_iter(text).map(|s| s.as_str()).collect();
    let mut sub_vec = Vec::new();
    for pair in sub {
        sub_vec.push(pair.split(' ').collect::<Vec<_>>());
    } 
    
    let mut possible: bool = true;

    for elem in sub_vec {
        if let [digit, color] = elem[..] {
            let digit = digit.parse::<i32>().unwrap();
            if digit > 14 {
                possible = false;
            }
            if (color == "red" && digit as i32 > red_rule) || (color == "green" && digit as i32 > green_rule) || (color == "blue" && digit as i32 > blue_rule) {
                possible = false;
            }
        }
    }

    if possible {
        answer += game_id;
    }
    answer
}
fn main() {
    
    

    let mut ids_sum = 0;

    for line in read_to_string("text.txt").unwrap().lines() {
        ids_sum += id_sum(&line.to_string());
    }
    println!("{}", ids_sum);
}