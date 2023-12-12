// В цикле берем первый char и, если его значение из мапы меньше 0, то заносим в st (стек), иначе false
// Затем берем следующий char, если он не отрицательный и сумма равна нулю, то дальше
// Если сумма не равна нулю, значит это не пара и false
use std::collections::HashMap;

fn valid_parenthenses(par_string: String) -> bool {
    let map_of_chars: HashMap<char, i32> = HashMap::from([
        ('(', -1),
        ('{', -2),
        ('[', -3),
        (')', 1),
        ('}', 2),
        (']', 3),
    ]);
    let mut st = Vec::new();
    for bracket in par_string.chars() {
        if map_of_chars[&bracket] < 0 {
            st.push(bracket);
        } else {
            if st.len() == 0 {
                return false;
            }
            let top = st.pop().unwrap();
            if map_of_chars[&bracket] + map_of_chars[&top] != 0 {
                return false;
            }
        }
    }
    if st.len() == 0 {
        return true;
    }
    return false;
}

fn main() {
    println!("{}", valid_parenthenses("([])".to_string()));
}
