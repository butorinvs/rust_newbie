/*
 Дана строка:
let txt: &str = "12345";

Получите первый и последний символ этой строки в следующем виде:
"15"*/
fn main() {
    let mut input = String::new();
    let _result: usize = std::io::stdin().read_line(&mut input).expect("");
    let first = input.trim().chars().next();
    let last = input.trim().chars().last();
    let mut out = String::new();
    out.push(first.unwrap());
    out.push(last.unwrap());
    println!("{:?}", out);
}
