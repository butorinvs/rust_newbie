fn input() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let mut str3 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    std::io::stdin().read_line(&mut str2).expect("ERR");
    std::io::stdin().read_line(&mut str3).expect("ERR");
    let nalog: f64 = str1.trim().parse::<f64>().unwrap();
    let power: u16 = str2.trim().parse::<u16>().unwrap();
    let month: u8 = str3.trim().parse::<u8>().unwrap();
    calc_tax(nalog, power, month);
}

fn calc_tax(nalog: f64, power: u16, month: u8) {
    let cost: f64 = nalog * power as f64 * (month as f64) / 12.0;
    print_cost(cost);
}
fn print_cost(tax: f64) {
    println!("{:.2}", tax);
}
fn main() {
    input();
}
