/*
 Дано целое число, содержащее номер месяца от 1 до 12:
let num: u8 = 1;

Выведите название месяца, соответствующее этому числу.  */
fn main() {
    let mut input = String::new();
    let _result: usize = std::io::stdin().read_line(&mut input).expect("");
    let in_digit: u8 = input.trim().parse::<u8>().expect("ERR");
    match in_digit {
        1 => println!("Январь"),
        2 => println!("Февраль"),
        3 => println!("Март"),
        4 => println!("Апрель"),
        5 => println!("Май"),
        6 => println!("Июнь"),
        7 => println!("Июль"),
        8 => println!("Август"),
        9 => println!("Сентябрь"),
        10 => println!("Октябрь"),
        11 => println!("Ноябрь"),
        12 => println!("Декабрь"),
        _ => println!("Ошибка!"),
    }
}
