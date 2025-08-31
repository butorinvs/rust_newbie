fn get_numb() -> u32 {
    let mut str1 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    let numb: u32 = str1.trim().parse::<u32>().unwrap();
    numb
}

fn get_price() -> f64 {
    let mut str2 = String::new();
    std::io::stdin().read_line(&mut str2).expect("ERR");
    let price: f64 = str2.trim().parse::<f64>().unwrap();
    price
}

fn get_coffee_cost(cups: u32, cup_cost: f64) -> f64 {
    let free = cups / 9;
    (cups - free) as f64 * cup_cost
}

fn main() {
    let numb = get_numb();
    let price = get_price();
    let res = get_coffee_cost(numb, price);
    println!("{res:.2}");
}
