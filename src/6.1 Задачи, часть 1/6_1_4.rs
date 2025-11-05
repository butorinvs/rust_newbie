/*
Напишите программу, которая считывает четырехзначное число (u16) и выводит его цифры с конца, как показано в примере.

Sample Input:
1234

Sample Output:
Последняя цифра: 4
Третья цифра: 3
Вторая цифра: 2
Первая цифра: 1
*/
use std::io;
fn main() {
    let mut user_data_0 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_data_0)
        .expect("не удалось прочитать");
    let data0 = user_data_0.trim().parse::<u16>().unwrap();
    let thousand = data0 / 1000;
    let cent = (data0 % 1000) / 100;
    let dec = (data0 % 100) / 10;
    let one = data0 % 10;
    println!("Последняя цифра: {}", one);
    println!("Третья цифра: {}", dec);
    println!("Вторая цифра: {}", cent);
    println!("Первая цифра: {}", thousand);
}
