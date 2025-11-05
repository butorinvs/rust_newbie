/*
Вам необходимо создать программу, которая считывает вещественное число (масса космических тел в кг)
и выводит массу объекта в научной записи (E). Это позволит вам анализировать массу космических объектов
в удобной форме и с легкостью их интерпретировать.

Тестовые данные ✅

Sample Input:
481068.0

Sample Output:
4.81068E5
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let in_num1: f64 = user_name1.trim().parse().expect("Error");

    println!("{:E}", in_num1);
}
