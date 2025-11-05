/*
Напишите программу, которая считывает 10 строковых значений и выводит их, как показано в примерах.

Sample Input:
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

Sample Output:
Вы ввели: 0
Вы ввели: 1
Вы ввели: 2
Вы ввели: 3
Вы ввели: 4
Вы ввели: 5
Вы ввели: 6
Вы ввели: 7
Вы ввели: 8
Вы ввели: 9
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let string = std::io::stdin()
        .lines()
        .take(v[0].into())
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();

    for x in 0..v[0] {
        println!("Вы ввели: {}", string[x]);
    }
}
