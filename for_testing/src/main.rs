// fn divide(numerator: f64, denominator: f64) -> Option<f64> {
//     if denominator == 0.0 {
//         None
//     } else {
//         Some(numerator / denominator)
//     }
// }
use std::{collections::HashMap, f64::consts::E, borrow::BorrowMut};
fn main() {
    // let result = divide(2.0, 5.0);
    
    // match  result {
    //     Some(x) => println!("Result: {x}"),
    //     None => println!("Cannot divide by 0"),
    // }

    let mut factory: HashMap<String, Vec<String>> = HashMap::new();
    let employees = vec!["Alina".to_string(), "Eva".to_string()];
    let employees2 = vec!["Diva".to_string()];
    factory.insert("dep1".to_string(), employees);
    factory.insert("dep2".to_string(), employees2);

    let new_dep = "dep1".to_string();
    let new_emp = "Diva".to_string();
    
    // let employees = factory.get_key_value(&new_dep).unwrap();
    let employees = factory.entry(new_dep).or_insert(Vec::new());
    employees.push(new_emp);
    println!("{:?}", employees);

    // factory.insert(new_dep, new_emp);
        
        

    
    // let mut emp: &Vec<String> = factory
    //     .entry("dep2".to_string())
    //     .or_insert_with(Vec::new);
        
        
    //     emp.push("Diva".to_string());
        
    //     println!("{:?}", emp);

        // let mut employees = &mut emp;
        // employees.push("Diva".to_string());
        
        
        // println!("{:?}", employees);
    
    
    println!("{:?}", factory);
    
}
