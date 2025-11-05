/*
Напишите программу, которая считывает два вещественных числа и выводит основные арифметические операции для них:
сложение, вычитание, умножение, деление и нахождение остатка от деления. Для операций / и % результат вывести до 3 знаков,
а  остальные без дробной части. Гарантируется, что второе число не равно 0!

Тестовые данные ✅

Sample Input 1:
2.0
4.0

Sample Output 1:
2 + (4) = 6
2 - (4) = -2
2 * (4) = 8
2 / (4) = 0.500
2 % (4) = 2.000

Sample Input 2:
40.0
20.0

Sample Output 2:
40 + (20) = 60
40 - (20) = 20
40 * (20) = 800
40 / (20) = 2.000
40 % (20) = 0.000
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let in_num1: f64 = user_name1.trim().parse().expect("Error");
    let in_num2: f64 = user_name2.trim().parse().expect("Error");

    println!("{in_num1} + ({in_num2}) = {}", in_num1 + in_num2);
    println!("{in_num1} - ({in_num2}) = {}", in_num1 - in_num2);
    println!("{in_num1} * ({in_num2}) = {}", in_num1 * in_num2);
    println!("{in_num1} / ({in_num2}) = {:.3}", in_num1 / in_num2);
    println!("{in_num1} % ({in_num2}) = {:.3}", in_num1 % in_num2);
}
