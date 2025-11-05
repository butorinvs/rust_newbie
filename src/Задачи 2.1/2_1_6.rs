/*
Найдите факториал этого числа.
 */
fn main() {
    let mut num: u16 = 12;
    let mut fact: u64 = 1;
    while num > 0 {
        fact *= num as u64;
        num -= 1;
    }
    println!("{:?}", fact);
}
