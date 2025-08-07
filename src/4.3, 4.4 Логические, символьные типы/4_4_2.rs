/*
Напишите программу, которая считывает пять строк, представленные символами и выводит их в одну строку.
Тестовые данные ✅
Sample Input:

こ
ん
に
ち
は

Sample Output:
こんにちは
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let mut user_name3 = String::new();
    let mut user_name4 = String::new();
    let mut user_name5 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi1 = user_name1.trim();

    let _result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let emodzi2 = user_name2.trim();

    let _result3 = io::stdin()
        .read_line(&mut user_name3)
        .expect("не удалось прочитать");
    let emodzi3 = user_name3.trim();

    let _result4 = io::stdin()
        .read_line(&mut user_name4)
        .expect("не удалось прочитать");
    let emodzi4 = user_name4.trim();

    let _result5 = io::stdin()
        .read_line(&mut user_name5)
        .expect("не удалось прочитать");
    let emodzi5 = user_name5.trim();

    println!("{}{}{}{}{}", emodzi1, emodzi2, emodzi3, emodzi4, emodzi5);
}
