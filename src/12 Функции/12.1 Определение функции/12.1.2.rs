fn main() {
    let mut count = String::new();

    std::io::stdin().read_line(&mut count).expect("ERR");
    let count: u32 = count.trim().parse::<u32>().unwrap();
    for v in 0..count {
        print!("{})", v + 1);
        print_str();
    }
}
fn print_str() {
    println!(" была вызвана функция!")
}
