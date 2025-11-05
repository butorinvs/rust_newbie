/*

*/

fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] == 1 {
        println!("Принята монета номинала {}", v[0]);
    } else if v[0] == 2 {
        println!("Принята монета номинала {}", v[0]);
    } else if v[0] == 5 {
        println!("Принята монета номинала {}", v[0]);
    } else if v[0] == 10 {
        println!("Принята монета номинала {}", v[0]);
    } else {
        println!("Монеты такого номинала не принимаются")
    }
}
