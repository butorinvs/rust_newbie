/*
Напишите программу, которая считывает два целых числа a и b, а затем выводит квадрат всех целых чисел,
 расположенных между a и b (включая их самих), как показано в примерах.
Гарантируется, что a≠ba=b !


Sample Input 1:
-10
-5

Sample Output 1:
100
81
64
49
36
25

Sample Input 2:
-10
-20

Sample Output 2:
400
361
324
289
256
225
196
169
144
121
100
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] > v[1] {
        for i in v[1]..=v[0] {
            println!("{}", i * i);
        }
    } else {
        for i in v[0]..=v[1] {
            println!("{}", i * i);
        }
    }
}
