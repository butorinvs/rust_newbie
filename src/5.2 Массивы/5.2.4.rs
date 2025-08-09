/*
Напишите программу, которая считывает два целых числа.
Далее инициализирует массив нулей размерностью десять и заменяет элемент в массиве по указанному индексу (первое число)
на значение второго числа. После этого программа выводит получившийся массив.
Гарантируется, что первое число не превышает длину массива!

Sample Input:
0
-10

Sample Output:
[-10, 0, 0, 0, 0, 0, 0, 0, 0, 0]
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
    let index: usize = user_name1.trim().parse::<usize>().unwrap();
    let data: i32 = user_name2.trim().parse::<i32>().unwrap();
    // Инициализация выходного массива
    let mut out: [i32; 10] = [0; 10];
    out[index] = data;
    println!("{:?}", out);
}
