fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![23, 4, 55, 71, 1, 90];
        
    println!("The largest item is {}", largest(&number_list));

    let char_list = vec!['a', 'e', 'j'];

    println!("The largest item is {}", largest(&char_list));
}

