// Сложите значения этих символов как целые числа.
fn main() {
    let chr1: char = '8';
    let chr2: char = '2';
    let chr3: char = '3';
    let digit1: u32 = chr1.to_digit(10).expect("Not a digit");
    let digit2: u32 = chr2.to_digit(10).expect("Not a digit");
    let digit3: u32 = chr3.to_digit(10).expect("Not a digit");
    let summ: u32 = digit1 + digit2 + digit3;
    println!("{}", summ);
}
