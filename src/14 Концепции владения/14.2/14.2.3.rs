/*
В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать строку и вывести является ли оно числом или нет:

    Если строка является числом, вывести {} является числом;
    Иначе вывести {} не является числом.

Проверку числа реализуйте в функции is_number() при этом саму передачу осуществлять с помощью clone()
 без использования дополнительной переменной или ссылки!

Sample Input:
0

Sample Output:
0 является числом
 */
fn is_number(str: String) -> bool {
    str.trim().parse::<f64>().is_ok()
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("ERR");
    let result = is_number(input.clone());
    if result {
        println!("{} является числом", input.trim());
    } else {
        println!("{} не является числом", input.trim());
    }
}
