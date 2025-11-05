//Дана некоторая строка:
//let txt: &str = "abcde";
//Переберите и выведите в консоль по очереди все ее символы.
fn main() {
    let txt: &str = "abcde";
    let lenght: usize = txt.len();
    for n in 0..lenght {
        println!("{}", &txt[n..n + 1]);
    }
}
