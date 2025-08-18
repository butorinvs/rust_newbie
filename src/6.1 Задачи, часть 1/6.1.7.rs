/*
Напишите программу, которая считывает два (u16) числа: количество квартир на каждом этаже и номер квартиры.
Затем программа определяет, на каком этаже этого дома находится квартира с заданным номером.
Предполагается, что в доме нет строительных особенностей, таких как пропуски этажей или квартир, а также что нумерация квартир начинается с 1.
Гарантируется, что номер квартиры не превышает общее количество квартир в доме.

Sample Input:
4
1

Sample Output:
Квартира с номером 1 находится на 1 этаже
*/
use std::io;
fn main() {
    let mut in1 = String::new();
    let mut in2 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut in1)
        .expect("не удалось прочитать");
    let _result1 = io::stdin()
        .read_line(&mut in2)
        .expect("не удалось прочитать");
    let count_flour = in1.trim().parse::<u16>().unwrap();
    let numb = in2.trim().parse::<u16>().unwrap();
    let count = (numb - 1) / count_flour + 1;
    println!("Квартира с номером {} находится на {} этаже", numb, count);
}
/*
fn main() {
    let v = std::io::stdin()
                   .lines()
                   .take(2)
                   .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
                   .collect::<Vec<_>>();
    print!("Квартира с номером {} находится на {} этаже", v[1], (v[1]-1) / v[0] + 1);
}
 */
