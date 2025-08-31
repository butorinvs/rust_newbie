/*
При покупке каждых 8 чашек кофе покупатель получает ещё одну бесплатно.

В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать количество покупаемых чашек кофе (u32) и цену за одну чашку (f64), а затем вывести (до 2 знаков) сумму к оплате.

Нахождение итоговой суммы реализуйте в функции get_сoffee_сost() !

Sample Input:

1
350.5

Sample Output:
350.50
*/
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
    let mut itogo = 0.0;
    let free = cups / 9;
    itogo = (cups - free) as f64 * cup_cost;
    itogo
}

fn main() {
    let numb = get_numb();
    let price = get_price();
    let res = get_coffee_cost(numb, price);
    println!("{:.2}", res);
}
