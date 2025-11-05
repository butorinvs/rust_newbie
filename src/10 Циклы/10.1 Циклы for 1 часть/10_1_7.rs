/*
Напишите программу, которая считывает два целых числа start и end,
а затем выводит результат произведения номера итерации на 2 для диапазона start..end, как показано в примерах.

Sample Input:
-10
-5

Sample Output:
-10 * 2 = -20
-9 * 2 = -18
-8 * 2 = -16
-7 * 2 = -14
-6 * 2 = -12
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for i in v[0]..v[1] {
        println!("{} * 2 = {}", i, i * 2);
    }
}
