/*В редакторе кода представлен кортеж с числами. Данные значения были получены по сети,
но во время передачи произошел разрыв соединения. Дополните программу считыванием пяти вещественных чисел.
Замените каждое старое значение на новое, начиная с первого элемента, и выведите целые части у чисел в кортеже, как показано в примере.
Тестовые данные ✅

Sample Input 1:
20
55
-30
11
-100

Sample Output 1:
20, 55, -30, 11, -100, 0

Sample Input 2:
1.5
0.0
6.89
-4.5
-10

Sample Output 2:
1, 0, 6, -4, -10, 0
*/

use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let mut user_name3 = String::new();
    let mut user_name4 = String::new();
    let mut user_name5 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let _result3 = io::stdin()
        .read_line(&mut user_name3)
        .expect("не удалось прочитать");
    let _result4 = io::stdin()
        .read_line(&mut user_name4)
        .expect("не удалось прочитать");
    let _result5 = io::stdin()
        .read_line(&mut user_name5)
        .expect("не удалось прочитать");
    let mut korteg = (0, 0, 0, 0, 0, 0);
    korteg.0 = user_name1.trim().parse::<f64>().unwrap() as i32;
    korteg.1 = user_name2.trim().parse::<f64>().unwrap() as i32;
    korteg.2 = user_name3.trim().parse::<f64>().unwrap() as i32;
    korteg.3 = user_name4.trim().parse::<f64>().unwrap() as i32;
    korteg.4 = user_name5.trim().parse::<f64>().unwrap() as i32;
    println!(
        "{}, {}, {}, {}, {}, {} ",
        korteg.0, korteg.1, korteg.2, korteg.3, korteg.4, korteg.5
    );
}
