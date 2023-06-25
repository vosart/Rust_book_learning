fn main() {
    //variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECOND);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    println!("{}", tup.1);

    //massives
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    let c = [0; 10];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{}", b[2])

} 