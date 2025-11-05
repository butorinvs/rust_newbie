//  Слейте эти символы в число:
// 12345
fn main() {
    let chr1: char = '1';
    let chr2: char = '2';
    let chr3: char = '3';
    let chr4: char = '4';
    let chr5: char = '5';
    let txt = format!("{}{}{}{}{}", chr1, chr2, chr3, chr4, chr5);
    let digit: i32 = txt.parse().expect("Not a number");
    println!("{}", digit);
}
