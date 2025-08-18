/*

*/

fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    if v[0] > 0.0 {
        println!("Число {} является положительным", v[0]);
    } else {
        println!("Число {} является отрицательным", v[0]);
    }
}
