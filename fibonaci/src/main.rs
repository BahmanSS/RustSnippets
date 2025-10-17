use std::io;

fn main() {
    println!("Введите значение n: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("ERR");

    let n: u32 = n.trim().parse().expect("Введите числовое значение >= 0!");

    println!("{n} число Фибоначи: {}", fib(n));
    
}

fn  fib( n: u32) -> u32 {
    if n==0 { return 0; }
    else if n == 1 { return 1; }

    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _i in 2..=n {
        b = a + b;
        a = b - a;
    }
    b
}
