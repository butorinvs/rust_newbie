/*
В редакторе кода представлен массив целых чисел. Дополните код и считайте целое число,
 а затем выведите сумму, разность, произведение предыдущего и следующего элемента по этому индексу, как показано в примере.

Гарантируется, что считанное число всегда больше нуля и не превышает индекса предпоследнего элемента!

Sample Input:
1

Sample Output:
3
-13
-40
*/
use std::io;

fn main() {
    let arr = [-5, 1, 8, 2, 30, 4000, 500000];
    let mut data = String::new();
    let _result = io::stdin().read_line(&mut data).expect("ошибка");
    let index: usize = data.trim().parse::<usize>().unwrap();
    let sum = arr[index - 1] + arr[index + 1];
    let sub = arr[index - 1] - arr[index + 1];
    let mult = arr[index - 1] * arr[index + 1];
    println!("{}", sum);
    println!("{}", sub);
    println!("{}", mult);
}
