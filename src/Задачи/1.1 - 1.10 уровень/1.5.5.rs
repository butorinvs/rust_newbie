/* Дана некоторая строка:
let txt: &str = "123456789";

Переберите и выведите в консоль по очереди все символы с конца строки. */

fn main() {
    let txt: &str = "123456789";
    let mut txt2: String = String::new();
    for n in (0..txt.len()).rev() {
        txt2.push_str(&txt[n..n + 1]);
    }
    println!("{:?}", txt2);
}
