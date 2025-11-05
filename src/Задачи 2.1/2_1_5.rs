/*
 Дано некоторое целое число:
let num: u16 = 12345;

Найдите сумму всех его четных цифр. */
fn main() {
    let num: u16 = 2222;
    let txt: String = num.to_string();
    let vec: Vec<u16> = txt
        .chars()
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .collect();
    let mut summ: u16 = 0;
    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            summ += vec[i];
        }
    }
    println!("{:?}", summ);
}
