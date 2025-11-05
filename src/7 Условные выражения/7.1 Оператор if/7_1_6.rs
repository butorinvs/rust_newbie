/*

*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(3)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] == v[1] {
        println!("Числа неразличны");
    } else if v[1] == v[2] {
        println!("Числа неразличны");
    } else if v[0] == v[2] {
        println!("Числа неразличны");
    } else {
        println!("Числа различны");
    }
}
