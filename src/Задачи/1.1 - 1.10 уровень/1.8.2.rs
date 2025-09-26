// "abcde"

// Переберите и выведите в консоль по очереди все символы с конца строки.
fn main() {
    let txt: &str = "abcde";
    for n in (0..txt.len()).rev() {
        println!("{}", &txt.chars().nth(n).expect("No char"));
    }
}
