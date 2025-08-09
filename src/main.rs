use std::io;

fn main() {
    let arr = [-5, 1, 8, 2, 30, 4000, 500000];
    let mut data = String::new();
    let _result = io::stdin().read_line(&mut data).expect("ошибка");
    let index: usize = data.trim().parse::<usize>().unwrap();
    let sum = arr[index - 1] + arr[index + 1];
    let sub = arr[index - 1] - arr[index + 1];
    let mult = arr[index - 1] * arr[index + 1];
    println!("{}", sum);
    println!("{}", sub);
    println!("{}", mult);
}
