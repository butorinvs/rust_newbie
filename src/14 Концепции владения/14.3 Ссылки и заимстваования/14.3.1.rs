/*

*/

fn main() {
    let mut str: String = String::new();
    std::io::stdin().read_line(&mut str).expect("ERR");

    let addr = &str;
    println!("{:p}", addr);
}
