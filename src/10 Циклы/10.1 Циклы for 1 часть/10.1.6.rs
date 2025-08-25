/*
Напишите программу, которая считывает строку,
целое неотрицательное число n (u8) и выводит считанную строку 0..n раз с указанием номера итерации, как показано в примерах.

Sample Input:
Am I coder yet?
3

Sample Output:
0: Am I coder yet?
1: Am I coder yet?
2: Am I coder yet?
*/
fn main() {
    let string = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..v[0] {
        println!("{}: {}", i, string[0]);
    }
}
