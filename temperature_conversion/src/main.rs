use std::io;
fn main() {
    println!("Введите температуру по Фаренгейту: ");

    let mut fahr = String::new();

    io::stdin()
        .read_line(& mut fahr)
        .expect("Error");

    let fahr: f64 = fahr.trim().parse().expect("Введите числовое значение!");
    let cels: f64 = (fahr - 32.0) / 1.8;

    println!("Температура по Цельсию: {cels:.2}");
}
