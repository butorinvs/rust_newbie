use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let kilo = user_data_0.trim().parse::<f64>().unwrap();
    let mile = kilo * 0.6214;
    let yard = kilo * 1093.61;
    let foot = kilo * 3280.84;
    let inch = kilo * 39370.1;
    println!("{} км = {:.3} миль", kilo, mile);
    println!("{} км = {:.3} ярдов", kilo, yard);
    println!("{} км = {:.3} футов", kilo, foot);
    println!("{} км = {:.3} дюймов", kilo, inch);
}