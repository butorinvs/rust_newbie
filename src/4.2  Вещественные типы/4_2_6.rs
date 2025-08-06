/*
Напишите программу, которая считывает два вещественных числа (килограммы и фунты) с консоли и выводит для первого фунты,
а для второго килограммы. Знаков после запятой достаточно 3.
1 фунт = 0.454 кг, а 1 кг = 2.205 фунта

Тестовые данные ✅

Sample Input:
2.0
4.0

Sample Output:
2 kg = 4.410 lbs
4 lbs = 1.816 kg
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let in_num1: f64 = user_name1.trim().parse().expect("Error");

    let result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let in_num2: f64 = user_name2.trim().parse().expect("Error");
    println!("{} kg = {:.3} lbs", in_num1, in_num1 * 2.205);
    println!("{} lbs = {:.3} kg", in_num2, in_num2 * 0.454);
}
