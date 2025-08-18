/*

*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(3)
        .map(|x| x.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    
    if v[0] < v[1] && v[1] < v[2] {
        println!("{:.1}, {:.1}, {:.1}", v[0], v[1], v[2])
    } else if v[1] < v[0] && v[0] < v[2] {
        println!("{:.1}, {:.1}, {:.1}", v[1], v[0], v[2])
    } else if v[1] < v[2] && v[2] < v[0] {
        println!("{:.1}, {:.1}, {:.1}", v[1], v[2], v[0])
    } else if v[2] < v[0] && v[0] < v[1] {
        println!("{:.1}, {:.1}, {:.1}", v[2], v[0], v[1])
    } else if v[2] < v[1] && v[1] < v[0] {
        println!("{:.1}, {:.1}, {:.1}", v[2], v[1], v[0])
    }
    if v[0] < v[2] && v[2] < v[1] {
        println!("{:.1}, {:.1}, {:.1}", v[0], v[2], v[1])
    }
}
