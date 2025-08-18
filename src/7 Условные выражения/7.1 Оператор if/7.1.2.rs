/*

*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    if v[0] > v[1] {
        println!("Из {:.3} и {:.3} больше {:.3}", v[0], v[1], v[0]);
    } else {
        println!("Из {:.3} и {:.3} больше {:.3}", v[0], v[1], v[1]);
    }
}
