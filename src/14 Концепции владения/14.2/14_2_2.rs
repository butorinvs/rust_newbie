fn main() {
    let mut str = String::new();

    std::io::stdin().read_line(&mut str).expect("ERR");

    let str2 = str.capacity();
    let str3 = str.len();
    println!("capacity = {}", str2);
    println!("length = {}", str3);
}
