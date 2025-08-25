fn main() {
    let stop = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();

    let mut input = String::new();
    //let mut vector: Vec<f32> = Vec::new();
    let mut summa: f64 = 0.0;
    while input.trim() != stop[0] {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Ошибка при чтении ввода.");
        // Надо проверить стоп слово на число
        //let stop2 = input.trim().to_string();
        if input.trim().to_string() == stop[0] {
            break;
        }
        let num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        summa += num;
        //println!("Вы ввели число: {:.1}", num);
        //vector.push(num);
    }
    summa -= -0.0;
    println!("{:.1}", summa);
}
