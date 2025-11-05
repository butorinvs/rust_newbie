/*
В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать в массив 10 целых чисел и вывести максимальное и минимальное значения в виде сообщений:
max:{}
min:{}
Нахождение максимального элемента реализуйте в функции max(), а минимального в min()!

Sample Input:
-9
-92
96
-5
-77
-5
124
34
-122
45

Sample Output:
max:124
min:-122
 */
fn get_input() -> Vec<i32> {
    let arr = std::io::stdin()
        .lines()
        .take(10)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    arr
}
fn max(array: Vec<i32>) -> i32 {
    *array.iter().max().unwrap()
}
fn min(array: Vec<i32>) -> i32 {
    *array.iter().min().unwrap()
}

fn main() {
    let number = get_input();
    let max_digit: i32 = max(number.clone());
    println!("max:{}", max_digit);
    let min_digit: i32 = min(number.clone());
    println!("min:{}", min_digit);
}
