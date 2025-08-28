/*
Напишите программу, которая считывает три значения:

    Правильный пароль (String);
    Количество попыток (u8);
    "Бесконечно" вводимое пользователем значение (String);

Программа должна выводить на каждую попытку ввода, предоставлен ли пользователю доступ Доступ предоставлен или нет Неверный пароль. Если количество попыток превысит установленное значение, вывести Слишком много попыток, пожалуйста повторите позже.

Sample Input:
12345
3
1
1234
12345

Sample Output:
Неверный пароль
Неверный пароль
Доступ предоставлен
*/
fn main() {
    let mut pass = String::new();
    let mut count = String::new();
    let mut input = String::new();

    std::io::stdin().read_line(&mut pass).expect("ERR");
    std::io::stdin().read_line(&mut count).expect("ERR");
    let count: u8 = count.trim().parse::<u8>().unwrap();
    let mut flag = false;
    for _ in 0..count {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("ERR");
        if input.trim() == pass.trim() {
            println!("Доступ предоставлен");
            flag = true;
            break;
        } else {
            println!("Неверный пароль");
        }
    }
    if !flag {
        println!("Слишком много попыток, пожалуйста повторите позже");
    }
}
