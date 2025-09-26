/* Дано некоторое число:
let num: u16 = 1234567;

Переверните его:
7654321 */
fn main() {
    let num: u32 = 1234670;
    let reversed: String = num.to_string().chars().rev().collect();
    let rev = reversed.parse::<u32>().expect("  Not a digit");
    println!("{}", rev);
}
