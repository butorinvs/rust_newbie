/*
Напишите программу, которая считывает стоп строку и вещественные числа, а после ввода сигнальной (стоп) строки выводит (до 1 знака) их сумму.

Гарантируется, что стоп строка не равна пробельным символам!


Sample Input:
stop
1
2
3
stop

Sample Output:
6.0
*/
fn main() {
    let stop = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();

    let mut input = String::new();
    //let mut vector: Vec<f32> = Vec::new();
    let mut summa: f64 = 0.0;
    while input.trim() != stop[0] {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка при чтении ввода.");
        //println!("Содержимое input: {:#?}", input);
        // Надо проверить стоп слово на число
        //let stop2 = input.trim().to_string();
        if input.trim().to_string() == stop[0] {
            break;
        }
        let num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        summa += num;
        //println!("Вы ввели число: {:.1}", num);
        //vector.push(num);
    }
    summa -= -0.0;
    println!("{:.1}", summa);
}
/*
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap);

    let stop = lines
        .next()
        .unwrap();

    let sum = lines
        .take_while(|s| s != &stop)
        .map(|s| s.parse::<f64>().unwrap())
        .sum::<f64>();

    println!("{sum:.1}");
}
*/
/*
fn main() {
    let mut sum: f64 = 0.;
    let mut stop = String::new();
    let mut buf= String::new();
    std::io::stdin().read_line(&mut stop).expect("input error");
    std::io::stdin().read_line(&mut buf).expect("input error");
    while  buf != stop{
        sum += buf.trim().parse::<f64>().expect("parsing number error");
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("input error");

    }
    println!("{sum:.1}");
}
*/
