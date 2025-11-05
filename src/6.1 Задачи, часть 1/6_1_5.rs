/*
Напишите программу, которая считывает расстояние (f64) в километрах и конвертирует его в мили, ярды, футы и дюймы. Достаточно вывести до 3 знаков.

1 километр = 0.621371 миль

1 километр = 1093.61 ярдов

1 километр = 3280.84 футов

1 километр = 39370.1 дюймов

Sample Input:
10

Sample Output:
10 км = 6.214 миль
10 км = 10936.100 ярдов
10 км = 32808.400 футов
10 км = 393701.000 дюймов
*/

use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let kilo = user_data_0.trim().parse::<f64>().unwrap();
    let mile = kilo * 0.621371;
    let yard = kilo * 1093.61;
    let foot = kilo * 3280.84;
    let inch = kilo * 39370.1;
    println!("{} км = {:.3} миль", kilo, mile);
    println!("{} км = {:.3} ярдов", kilo, yard);
    println!("{} км = {:.3} футов", kilo, foot);
    println!("{} км = {:.3} дюймов", kilo, inch);
}
