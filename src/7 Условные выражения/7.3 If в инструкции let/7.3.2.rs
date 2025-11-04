/*!
Напишите программу, которая считывает номер месяца (u8)
в диапазоне от 1 до 12 (1 январь, 2 февраль и т. д.)
и выводит количество дней в этом месяце для невисокосного года.
*/

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let month: u8 = input.trim().parse().unwrap();
    let season = if month <= 2 {
        "Зима"
    } else if month <= 5 {
        "Весна"
    } else if month <= 8 {
        "Лето"
    } else if month <= 11 {
        "Осень"
    } else {
        "Зима"
    };
    println!("{}", season);
}
