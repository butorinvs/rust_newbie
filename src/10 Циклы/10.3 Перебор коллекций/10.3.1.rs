/*
Напишите программу, которая считывает натуральное значение n (u8) и столько же вводимых пользователем чисел, а затем выводит (до 1 знака) для получившейся последовательности среднее значение в виде сообщения Среднее значение: {}.

Sample Input 1:
3
1
2
3
Sample Output 1:
Среднее значение: 2.0

Sample Input 2:
6
1.5
2.2
3.1
-1.7
-0.5
10.6
Sample Output 2:
Среднее значение: 2.5
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let real = std::io::stdin()
        .lines()
        .take(v[0].into())
        .map(|x| x.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let mut summa: f64 = 0.0;
    for x in 0..v[0] {
        summa += real[x]
    }
    println!("Среднее значение: {:.1}", summa / v[0]);
}
