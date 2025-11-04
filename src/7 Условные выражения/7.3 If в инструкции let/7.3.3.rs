/*!
В редакторе кода представлен кортеж значений.
Дополните код и выведите (до 2 знаков) сумму всех положительных значений.
*/

fn main() {
    let tup = (1, 3.14, -12.3, -50, 100, 250, -4, 7.6);
    let mut summa = 0.0;
    let a = tup.0 as f64;
    let b = tup.1 as f64;
    let c = tup.2 as f64;
    let d = tup.3 as f64;
    let e = tup.4 as f64;
    let f = tup.5 as f64;
    let g = tup.6 as f64;
    let h = tup.7 as f64;
    if a > 0.0 {
        summa += a;
    };
    if b > 0.0 {
        summa += b;
    };
    if c > 0.0 {
        summa += c;
    };
    if d > 0.0 {
        summa += d;
    };
    if e > 0.0 {
        summa += e;
    };
    if f > 0.0 {
        summa += f;
    };
    if g > 0.0 {
        summa += g;
    };
    if h > 0.0 {
        summa += h;
    };

    println!("{:.2}", summa);
}
