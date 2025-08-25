/*
Напишите программу, которая считывает два целых числа a и b,
а затем выводит в порядке возрастания все целые числа, расположенные между a и b (включая их самих), как показано в примерах.

Гарантируется, что a≠ba=b !

Sample Input:
-5
-10

Sample Output:
-10
-9
-8
-7
-6
-5
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] > v[1] {
        for i in v[1]..=v[0] {
            println!("{}", i);
        }
    } else {
        for i in v[0]..=v[1] {
            println!("{}", i);
        }
    }
}
