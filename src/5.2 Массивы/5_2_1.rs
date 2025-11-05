/*
В редакторе кода представлен массив значений. Дополните код и считайте целое число (usize),
 а затем выведите элемент массива по этому индексу, как показано в примере. Гарантируется, что считанное число не превышает длины массива!

Sample Input:
0

Sample Output:
-1
*/

use std::io;
fn main() {
    let arr = [-1, 0, 1, 2, 30, 4, 500];
    let mut user_name1 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let index: usize = user_name1.trim().parse::<usize>().unwrap();
    println!("{}", arr[index]);
}
