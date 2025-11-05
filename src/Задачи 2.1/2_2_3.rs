/*
Найдите сумму первой и последней цифры этого числа.
 */
fn main() {
    let num: u16 = 62345;
    let txt: String = num.to_string();
    let vec: Vec<u16> = txt
        .chars()
        .map(|c| c.to_string().parse::<u16>().unwrap())
        .collect();
    let mut summ: u16 = 0;
    summ += vec.first().unwrap() + vec.last().unwrap();
    println!("{:?}", summ);
}
