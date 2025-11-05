/*
Напишите программу, которая считывает четыре значения:

    Целое число n (i8);
    Первая начальная позиция p1 (i8);
    Вторая начальная позиция p2 (i8);
    Количество обмениваемых бит len (i8).

Программа должна поменять у n местами два участка бит, начало которых определяется вторым (p1) и третьим (p2) считанными значениями размерности len.
В конце вывести получившееся число в двоичном и десятичном представлении:

До перестановки
Двоичный вид: {:08b}
Десятичный вид: {}

После перестановки
Двоичный вид: {:08b}
Десятичный вид: {}

Гарантируется, что два набора битов не пересекаются! Отсчет позиций производить справа с нуля! Вычисление нового числа реализуйте в функции swap() !

Sample Input:
15
0
4
4

Sample Output:
До перестановки
Двоичный вид: 00001111
Десятичный вид: 15

После перестановки
Двоичный вид: 11110000
Десятичный вид: -16
*/
fn swap(n: i8, p1: i8, p2: i8, len: i8) -> i8 {
    // работаем как с беззнаковыми 8-битными числами
    let x = n as u8;
    let mask = ((1u8 << len) - 1) as u8;

    let bits1 = (x >> p1) & mask;
    let bits2 = (x >> p2) & mask;

    let mut result = x;
    result &= !(mask << p1);
    result &= !(mask << p2);

    result |= bits2 << p1;
    result |= bits1 << p2;

    result as i8
}
fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let mut str3 = String::new();
    let mut str4 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    std::io::stdin().read_line(&mut str2).expect("ERR");
    std::io::stdin().read_line(&mut str3).expect("ERR");
    std::io::stdin().read_line(&mut str4).expect("ERR");
    let n: i8 = str1.trim().parse::<i8>().unwrap();
    let p1: i8 = str2.trim().parse::<i8>().unwrap();
    let p2: i8 = str3.trim().parse::<i8>().unwrap();
    let len: i8 = str4.trim().parse::<i8>().unwrap();

    println!("До перестановки");
    println!("Двоичный вид: {:08b}", n);
    println!("Десятичный вид: {}", n);
    let res = swap(n, p1, p2, len);
    print!("\n");
    println!("После перестановки");
    println!("Двоичный вид: {:08b}", res);
    println!("Десятичный вид: {}", res);
}
