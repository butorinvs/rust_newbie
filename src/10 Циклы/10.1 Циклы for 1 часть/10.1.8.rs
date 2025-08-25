/*
Напишите программу, которая считывает два целых числа start и end,
а затем выводит сумму сгенерированной последовательности диапазоном start..end, как показано в примерах.

Sample Input:
-10
-5

Sample Output:
-40
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut summ = 0;
    for i in v[0]..v[1] {
        summ += i;
    }
    println!("{}", summ);
}
