/*
 Дано некоторое целое число:
let num: u16 = 12345;

Выведите в консоль первые его три цифры.  */
fn main() {
    let num: u16 = 12345;
    let revers: Vec<_> = num.to_string().chars().collect(); // через вектор
    let mut rev: String = String::new();
    for c in 0..3 {
        rev.push(revers[c]);
    }
    let rev_digit: u16 = rev.parse().unwrap(); // обратно в число
    println!("{:?}", rev_digit);
}
