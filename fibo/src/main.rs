// число Фибоначчи для n-го элемента
// добавить потом чере вектор
use std::io;

fn main() {
    println!("Enter n:");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Wrong number");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Wrong number!"),
    };
    let answer = fibo(n);
    println!("Fibo {n} = {answer}");
}

fn fibo(n: u64) -> u64 {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut fibo_n = 0;
    let mut count = 2;
    while count <= n {
        let a1 = f1;
        let a2 = f2;
        fibo_n = a1 + a2;
        f2 = fibo_n;
        f1 = a2;
        count += 1;
    }
    fibo_n
}