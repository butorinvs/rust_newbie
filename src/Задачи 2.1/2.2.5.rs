/*
 Дано некоторое целое число:
let num: u16 = 12345;

Выведите в консоль все его цифры с конца.
 */
fn main() {
    let num: u32 = 12345;
    //let txt: String = num.to_string();
    let revers: Vec<_> = num.to_string().chars().rev().collect(); // через вектор
    //println!("{:?}", revers);
    let txt: String = revers.iter().collect(); // через строку
    //  println!("{:?}", txt);
    let swap: u32 = txt.parse().unwrap(); // обратно в число
    println!("{:?}", swap);
}
