/*
Напишите программу, которая считывает:

    Строку останова (String);
    "Бесконечно" вводимую температуру двигателя в °С (i32).

Программа должна выводить Термостат закрыт, если температура ниже нормы и Термостат открыт, если температура выше нормы.


Sample Input:

turn off
10
20
50
70
80
90
100
110
90
85
turn off

Sample Output:

Термостат закрыт
Термостат закрыт
Термостат закрыт
Термостат закрыт
Термостат закрыт
Термостат закрыт
Термостат открыт
Термостат открыт
Термостат закрыт
Термостат закрыт
*/
fn main() {
    let stop = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    let mut input = String::new();
    let mut vector: Vec<String> = Vec::new();
    while input.trim() != stop[0] {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка при чтении ввода.");
        // Делаем цифру из input
        let num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if num > 90.0 {
            vector.push("Термостат открыт".to_string());
        } else {
            vector.push("Термостат закрыт".to_string());
        }
    }
    for v in vector {
        println!("{v}");
    }
}
