// {"Department1": ["Tom", "Sally", "Luis"], "Department2": ["Anastasia", "Alina", "Divini"]}
use std::collections::HashMap;
use std::io;

fn add_empl(factory: &mut HashMap<String, Vec<String>>, 
            department: String, 
            employee: String) -> &mut HashMap<String, Vec<String>>{
    
    let employees = factory.entry(department).or_insert(Vec::new());
    employees.push(employee);
    factory
}

fn dep_employees_list<'a>(factory: &'a mut HashMap<String, Vec<String>>, department: &'a String) -> Vec<&'a String> {
    let mut employees: Vec<&String> = Vec::new();
    for emp in factory.get(department).unwrap() {
        employees.push(emp);
    }
    employees.sort();
    employees

}

fn main() {
    println!("[INSTRUCTION]");
    println!("1. add 'Person' to 'Departament'");
    println!("2. list dep 'Departament'");
    println!("3. list all\n");
    
    let mut factory: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Error!");
    
    let mut commands: Vec<String> = Vec::new();
    
    for word in input.split_whitespace() {
        commands.push(word.to_string());
    }
    if commands.len() < 2 && commands[0].to_string() != "exit" {
        println!("Enter parameters!");
        continue;
    }
    match commands[0].to_lowercase().trim() {
        "exit" => break,
        "list" => {
                match commands[1].to_lowercase().trim() {
                    "dep" => {
                        // TODO: make an uppercase
                        // fn make_ascii_titlecase(s: &mut str) {
                        //     if let Some(r) = s.get_mut(0..1) {
                        //         r.make_ascii_uppercase();
                        //     }
                        // }
                        for name in dep_employees_list(&mut factory, &commands[2].to_string()) {
                            println!("{}", name);
                        }
                        continue;
                    }
                    "all" => {
                        // TODO: избавиться от clone()
                        let mut factory_deps: Vec<String> = factory.clone().into_keys().collect();
                        factory_deps.sort_unstable();

                        for dep in factory_deps {
                            println!("[{}]", dep);
                                for name in dep_employees_list(&mut factory, &dep) {
                                    println!("{name}");
                                };
                                continue;
                            }
                            continue;
                        }
                        _ => {
                            println!("Enter option!");
                        continue;
                    }
                }
            },
            "add" => {
                add_empl(&mut factory, commands[3].to_string(), commands[1].to_string());
                continue;
            }
            _ => {
                println!("Wrong input!");
                continue;
            }
        }
    }
}
