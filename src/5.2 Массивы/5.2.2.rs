/*
Напишите программу, которая считывает 5 вещественных значений и 1 целое число (usize),
записывает первые пять значений в массив и выводит (до 2 знаков) элемент коллекции по индексу
последнего считанного числа, как показано в примере. Гарантируется, что последнее считанное число не превышает длины массива!

Sample Input:

1
2
3
4
5
0

Sample Output:
1.00
*/
use std::io;
fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let mut user_name3 = String::new();
    let mut user_name4 = String::new();
    let mut user_name5 = String::new();
    let mut user_name6 = String::new();

    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name3)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name4)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name5)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut user_name6)
        .expect("не удалось прочитать");
    // Инициализация
    let mut arr: [f64; 5] = [0.0; 5];
    arr[0] = user_name1.trim().parse::<f64>().unwrap();
    arr[1] = user_name2.trim().parse::<f64>().unwrap();
    arr[2] = user_name3.trim().parse::<f64>().unwrap();
    arr[3] = user_name4.trim().parse::<f64>().unwrap();
    arr[4] = user_name5.trim().parse::<f64>().unwrap();
    // Элемент индекса для вывода
    let index: usize = user_name6.trim().parse::<usize>().unwrap();
    println!("{:.2}", arr[index]);
}
