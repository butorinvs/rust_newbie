use std::io;

fn main() {
    let mut user_name1 = String::new();
    let mut user_name2 = String::new();
    let _result1 = io::stdin()
        .read_line(&mut user_name1)
        .expect("не удалось прочитать");
    let _in_num1: f64 = user_name1.trim().parse().expect("Error");

    let _result2 = io::stdin()
        .read_line(&mut user_name2)
        .expect("не удалось прочитать");
    let _in_num2: usize = user_name2.trim().parse().expect("Error");

    println!("{:._in_num2$}", _in_num1);
}
