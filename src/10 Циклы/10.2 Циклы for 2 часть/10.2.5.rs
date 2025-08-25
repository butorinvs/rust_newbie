/*
Напишите программу, которая считывает натуральное число n (u8) и столько же вводимых пользователем чисел, а затем выводит (до 1 знака) их сумму, как показано в примерах.
Sample Input 1:
1
1.5

Sample Output 1:
Сумма считанных чисел равна: 1.5

Sample Input 2:
2
-1.5
-2.7

Sample Output 2:
Сумма считанных чисел равна: -4.2
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let index = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut summa: f64 = 1.0;
    for i in (v[0]..=v[1]).step_by(index[0]) {
        summa *= i as f64;
    }
    println!("{:.1}", summa);
}
