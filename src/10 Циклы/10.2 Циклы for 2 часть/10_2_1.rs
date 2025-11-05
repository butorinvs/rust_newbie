/*
Напишите программу, которая считывает 10 строковых значений и выводит их, как показано в примерах.

Sample Input:
0
1
2
3
4
5
6
7
8
9

Sample Output:
Вы ввели: 0
Вы ввели: 1
Вы ввели: 2
Вы ввели: 3
Вы ввели: 4
Вы ввели: 5
Вы ввели: 6
Вы ввели: 7
Вы ввели: 8
Вы ввели: 9
*/
use std::io::BufRead;
fn main() {
    let v = std::io::stdin()
        .lock()
        .lines()
        .take(10)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    for line in v {
        println!("Вы ввели: {}", line);
    }
}
