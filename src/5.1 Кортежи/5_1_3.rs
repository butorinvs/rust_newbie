/*
Напишите программу, которая считывает пять строк, сохраняет их в той же последовательности в кортеж и затем выводит
получившуюся коллекцию с помощью спецификатора формата :?.

Sample Input:

Упаковка
строк
в
кортеж
!

Sample Output:
("Упаковка\n", "строк\n", "в\n", "кортеж\n", "!\n")
*/

use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let mut user_name3 = String::new();
    let mut user_name4 = String::new();
    let mut user_name5 = String::new();
    let _korteg: (&str, &str, &str, &str, &str);
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let _result3 = io::stdin()
        .read_line(&mut user_name3)
        .expect("не удалось прочитать");
    let _result4 = io::stdin()
        .read_line(&mut user_name4)
        .expect("не удалось прочитать");

    let _result5 = io::stdin()
        .read_line(&mut user_name5)
        .expect("не удалось прочитать");
    let korteg = (
        &user_name1,
        &user_name2,
        &user_name3,
        &user_name4,
        &user_name5,
    );
    println!("{:?}", korteg);
}
