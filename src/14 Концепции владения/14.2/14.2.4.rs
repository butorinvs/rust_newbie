/*
В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать натуральное число (u32) и вывести все его делители, не считая 1 и самого числа:

Считывание реализуйте в функции get_input() с возвратом считанного значения!

Sample Input:
8

Sample Output:
2 4
 */
//use std::intrinsics::sqrtf32;

fn get_input() -> String {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("ERR");
    input.to_string()
}

fn main() {
    let input = get_input();
    let numb: u32 = input.trim().parse::<u32>().unwrap();
    let mut arr: Vec<u32> = Vec::new();
    for i in 2..numb - 1 {
        if numb % i == 0 {
            arr.push(i);
        }
    }
    for i in 0..arr.len() {
        print!("{} ", arr[i]);
    }
}
