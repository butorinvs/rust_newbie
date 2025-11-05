/*
Напишите программу, которая считывает вещественное число и выводит (до 3 знаков) его дробную часть, используя оператор %.


Sample Input:
17.1235

Sample Output:
0.123
*/

use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let data0 = user_data_0.trim().parse::<f64>().unwrap();
    let dec = data0 % 1.0;
    println!("{:.3}", dec);
}
