/*
Напишите программу, которая считывает целое неотрицательное число n (u16) и выводит c новой строки диапазон от 0 до n (невключительно), как показано в примерах.

Sample Input:
10

Sample Output:
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
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..v[0] {
        println!("{}", i);
    }
}
