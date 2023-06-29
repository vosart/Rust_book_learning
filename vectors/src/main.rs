fn main() {
    let v: Vec<i32> = Vec::new();

    let x = vec![1, 2, 3];

    println!("{:?}", x);

    let mut z = Vec::new();
    z.push(5);
    z.push(6);
    z.push(7);

    println!("{:?}", z);

    let v = vec![1, 2, 3, 4, 5, 6];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v1 = vec![1, 2, 3, 4, 5, 6];

    for i in &mut v1 {
        *i += 50;
    }
    println!("{:?}", v1);

    // vector с типом enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(11.45),
    ];

    println!("{:?}", row);

    let vec = vec![0; 5];
    println!("{:?}",vec);

}
