fn get_len_width() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let mut str3 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    std::io::stdin().read_line(&mut str2).expect("ERR");
    std::io::stdin().read_line(&mut str3).expect("ERR");
    let len: f64 = str1.trim().parse::<f64>().unwrap();
    let wid: f64 = str2.trim().parse::<f64>().unwrap();
    let cost: f64 = str3.trim().parse::<f64>().unwrap();
    calc_cost(len, wid, cost);
}

fn calc_cost(lenght: f64, width: f64, price: f64) {
    let cost: f64 = lenght * width * price;
    print_cost(cost);
}
fn print_cost(cost: f64) {
    println!("{:.2}", cost);
}
fn main() {
    get_len_width();
}
