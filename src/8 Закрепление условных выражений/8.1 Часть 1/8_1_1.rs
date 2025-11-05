/*
Напишите программу, которая считывает трехзначное число и выводит сумму чисел,
образованные его цифрами как показано в примерах.
*/

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("ошибка");

    let data = input.trim().parse::<i32>().unwrap();
    let sotni = data / 100;
    let desyatki = ((data / 10) % 10).abs();
    let edinicy = (data % 10).abs();
    let summa = sotni + desyatki + edinicy;
    println!("{}", summa);
}
