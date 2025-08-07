/*
Напишите программу, которая считывает строку, представленную в виде эмодзи и выводит ее обратно следующим образом:
Мое настроение: эмодзи

Тестовые данные ✅
Sample Input:
🥲

Sample Output:
Мое настроение: 🥲
*/

use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let mut user_name3 = String::new();
    let mut user_name4 = String::new();
    let mut user_name5 = String::new();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi1 = user_name1.trim();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi2 = user_name2.trim();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi3 = user_name3.trim();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi4 = user_name4.trim();

    let result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let emodzi5 = user_name5.trim();

    println!("{}{}{}{}{}", emodzi1, emodzi2, emodzi3, emodzi4, emodzi5);
}
