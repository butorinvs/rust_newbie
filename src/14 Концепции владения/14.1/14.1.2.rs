/*
В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считать радиус шара (f64) и вывести (до 3 знаков) его объем:

V=43πr3V=34​πr3;

где r - радиус шара.

Для достижения достаточной точности используете π=3.141592653589793π=3.141592653589793.

Sample Input:
0

Sample Output:
0.000
 */
fn newton(r: u32, x:f64) -> f64 {
    let pi = 3.141592653589793;
    let value = 4.0 * pi * r * r * r / 3.0;
    value
}
fn main() {
    let mut str1 = String::new();

    std::io::stdin().read_line(&mut str1).expect("ERR");

    let r: f64 = str1.trim().parse::<f64>().unwrap();

    let result = value_shere(r);
    println!("{result:.3}");
}
