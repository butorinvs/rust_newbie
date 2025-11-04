/*!
Напишите программу, которая считывает номер месяца (u8)
в диапазоне от 1 до 12 (1 январь, 2 февраль и т. д.)
и выводит количество дней в этом месяце для невисокосного года.
*/

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let month: u8 = input.trim().parse().unwrap();
    let days = if month == 2 {
        28
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    };
    println!("{}", days);
}
