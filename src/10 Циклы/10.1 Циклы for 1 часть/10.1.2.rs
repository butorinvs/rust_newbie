/*
Напишите программу, которая считывает целое число n и выводит с новой строки сообщение: Rustacean 🦀 n раз.
*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for _0 in 0..v[0] {
        println!("Rustacean 🦀");
    }
}
