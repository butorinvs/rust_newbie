/*
Напишите программу, которая считывает 5 целых чисел, записывает их в массив и выводит элементы коллекции
 по индексу считанных чисел, как показано в примере. Гарантируется, что числа не отрицательны и не превышают длины массива!
Sample Input 1:
0
1
2
1
0

Sample Output 1:
0, 1, 2, 1, 0

Sample Input 2:
4
0
3
2
1

Sample Output 2:
1, 4, 2, 3, 0
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
    // Инициализация
    let mut input: [u32; 5] = [0; 5];

    input[0] = user_name1.trim().parse::<u32>().unwrap();
    input[1] = user_name2.trim().parse::<u32>().unwrap();
    input[2] = user_name3.trim().parse::<u32>().unwrap();
    input[3] = user_name4.trim().parse::<u32>().unwrap();
    input[4] = user_name5.trim().parse::<u32>().unwrap();
    let index0: usize = input[0] as usize;
    let index1: usize = input[1] as usize;
    let index2: usize = input[2] as usize;
    let index3: usize = input[3] as usize;
    let index4: usize = input[4] as usize;
    let mut out: [u32; 5] = [0; 5];
    out[0] = input[index0];
    out[1] = input[index1];
    out[2] = input[index2];
    out[3] = input[index3];
    out[4] = input[index4];
    println!("{}, {}, {}, {}, {}", out[0], out[1], out[2], out[3], out[4]);
}
