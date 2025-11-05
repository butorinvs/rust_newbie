/*
В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать в массив 10 целых чисел и вывести с помощью заполнителя {:?} развернутую коллекцию.
Разворот массива реализуйте в функции reverse() !

Sample Input:
-11
75
81
-34
32
-79
29
-68
-114
8

Sample Output:
[8, -114, -68, 29, -79, 32, -34, 81, 75, -11]
*/
fn get_input() -> Vec<i32> {
    let arr = std::io::stdin()
        .lines()
        .take(10)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    arr
}
fn reverse(mut array: Vec<i32>) -> Vec<i32> {
    array.reverse();
    array
}
fn main() {
    let number = get_input();
    let arr: Vec<i32> = reverse(number);
    println!("{:?}", arr);
}
