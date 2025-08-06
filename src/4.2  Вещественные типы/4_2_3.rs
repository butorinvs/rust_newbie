/*
Напишите программу, которая считывает вещественное число и выводит его целое и дробную часть (до 3 знаков).
Например, если введено число 123.456789, программа должна вывести:
123
0.457

Тестовые данные ✅

Sample Input:
123.456789

Sample Output:
123
0.457
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let mut _in_num1: f64 = user_name1.trim().parse().expect("Error");
    let mut _in_num2: i32 = _in_num1 as i32;
    let mut _in_num3: f64 = _in_num2 as f64;
    _in_num1 -= _in_num3;
    println!("{}", _in_num2);
    println!("{:.3}", _in_num1);
}
