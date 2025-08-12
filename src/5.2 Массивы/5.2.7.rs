/*
В редакторе кода представлен массив значений. Дополните код и считайте два целых числа.
Далее поменяйте местами значения по этим индексам, а затем выведите (до 9 знаков) получившийся массив,
как показано в примере. Гарантируется, что считанные числа (индексы) заданы корректно!

Sample Input:
0
6

Sample Output:
[0.000051789, 11.100000000, 2.000000000, -7.123000000, 0.125000000, 0.000000000, -621.500000000]
*/

use std::io;

fn main() {
    let mut arr = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];
    let mut in0 = String::new();
    let mut in1 = String::new();

    let _result = io::stdin().read_line(&mut in0).expect("ошибка");
    let _result = io::stdin().read_line(&mut in1).expect("ошибка");

    let index0: usize = in0.trim().parse::<usize>().unwrap();
    let index1: usize = in1.trim().parse::<usize>().unwrap();
    let arr_out: [f64; 7] = [0.0; 7];
    let between0 = arr[index0];
    let between1 = arr[index1];
    arr[index1] = between0;
    arr[index0] = between1;
    println!("{:.9?}", arr);
}
