/*
Напишите программу, которая считывает трехзначное положительное число (u16) и выводит сумму его цифр.

Чтобы найти сумму цифр трехзначного числа, можно разложить число на отдельные цифры и затем сложить их.
Тестовые данные ✅

Sample Input:
123

Sample Output:
6
*/
use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let data0 = user_data_0.trim().parse::<u16>().unwrap();
    let cent = data0 / 100;
    let dec = (data0 % 100) / 10;
    let one = data0 % 10;
    println!("{}", cent + dec + one);
}
