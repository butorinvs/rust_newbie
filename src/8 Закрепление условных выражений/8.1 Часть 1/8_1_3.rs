/*
Два друга играли в разведчиков и кодировали сообщения собственным шифром.
Фрагмент кодовой таблицы приведен ниже:
О 	Н 	Г 	И
1 	2 	3 	4

Напишите программу, которая считывает четырехзначное целое число и
выводит расшифрованное сообщение. Гарантируется, что буквы в нем не повторяются.
*/
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERR");
    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();
    let mut txt: Vec<&str> = Vec::new();
    for i in 0..digits.len() {
        if digits[i] == 1 {
            txt.push("О");
        } else if digits[i] == 2 {
            txt.push("Н");
        } else if digits[i] == 3 {
            txt.push("Г");
        } else if digits[i] == 4 {
            txt.push("И");
        };
    }
    println!("{}", txt.join(""));
}
