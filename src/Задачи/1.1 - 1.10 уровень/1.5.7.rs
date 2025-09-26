/* Слейте эти символы в строку:
std::string "abc" */
fn main() {
    let chr1: char = 'a';
    let chr2: char = 'b';
    let chr3: char = 'c';
    let mut txt: String = String::new();
    txt.push(chr1);
    txt.push(chr2);
    txt.push(chr3);
    println!("{}", txt);
}
