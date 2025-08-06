/*
Вы оказались в странном мире, где правила математики работают немного иначе. Здесь банкоматы сломались,
и теперь они пополняют карты только купюрами в виде целых чисел, в то время как баланс вашей карты хранится
в виде вещественного числа.

Вам нужно создать программу, которая считывает текущий баланс (f64) и вносимую сумму (u32), а затем выводит
(до 1 знака) итоговый баланс после проведения операции пополнения.
Тестовые данные ✅
Sample Input:
1000.0
500

Sample Output:
1500.0
*/
use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _in_num1: f64 = user_name1.trim().parse().expect("Error");

    let _result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let mut _in_num2: u32 = user_name2.trim().parse().expect("Error");
    _in_num1 += _in_num2 as f64;
    println!("{:.}", _in_num1);
}
