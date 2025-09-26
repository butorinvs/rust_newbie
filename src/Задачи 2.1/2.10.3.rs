fn main() {
    let mut input = String::new();
    let _result: usize = std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Удаляем символ новой строки и пробелы
    let input = input.trim();

    let mut arr: [i32; 5] = input
        .chars()
        .filter_map(|c| c.to_digit(10)) // Фильтруем только цифры
        .map(|d| d as i32) // Преобразуем u32 в i32
        .take(5) // Берем только первые 5 цифр
        .collect::<Vec<i32>>()
        .try_into()
        .expect("Need exactly 5 digits");
    let temp = arr[0];
    arr[0] = arr[4];
    arr[4] = temp;
    println!("{:?}", arr);
}
