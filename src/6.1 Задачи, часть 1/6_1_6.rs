/*
Напишите программу, которая считывает целое число (u64), представляющее собой количество секунд, прошедших с начала суток, и выводит это время в формате: X сек = H час M минут S секунд.

Sample Input:
3665

Sample Output:
3665 сек = 1 час 1 минут 5 секунд
*/

use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let time = user_data_0.trim().parse::<u64>().unwrap();
    let sec = time % 60;
    let min = (time / 60) % 60;
    let hour = time / 3600;

    println!("{} сек = {} час {} минут {} секунд", time, hour, min, sec);
}
