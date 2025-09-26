// Найдите сумму цифр из этой строки.
fn main() {
    let txt: &str = "123456789";

    let mut summ: u32 = 0;
    for char in txt.chars() {
        if let Some(digit) = char.to_digit(10) {
            summ += digit;
        } else {
            eprintln!("Not a digit {}", char);
        }
    }
    println!("{}", summ);
}
