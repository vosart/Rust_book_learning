use std::collections::HashMap;
fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores);

    
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("{}", score);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    
    // let field_name = String::from("Fav color");
    // let field_value = String::from("Red");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // // field_name and field_value are invalid at this point, try using them and
    // // see what compiler error you get!
    // println!("{:?}", map);

    // Обновление данных в HashMap
    // 1. Перезапись старых значений
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 64);

    // println!("{:?}", scores);
    // // 2. Вставка, если значения по ключу не было
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);
    // 3. Создание нового значения, на основе старого
    let nums = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    let mut table = HashMap::new();

    for num in nums {
        println!("num: {}", num);
        println!("table0: {:?}", table);
        
        //let count = table.entry(num).or_insert(0);
        
        let count = table
            .entry(num)
            .and_modify(|e| {*e += 1})
            .or_insert(1);
        
        println!("count: {}", count);
        //*count += 1;
        //println!("count1: {}", count);
        println!("table1: {:?}", table);
    }
    
    println!("{:?}", table);
    // let v1: Vec<_> = table.iter().map(|(_, v)| v).collect();
    // println!("{:?}", v1.iter().max().unwrap());
    
}
