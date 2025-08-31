fn print_pmmm() {
    print!("+--");
}
fn print_p() {
    println!("+");
}
fn print_vss() {
    print!("|  ");
}
fn print_v() {
    println!("|");
}
fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    std::io::stdin().read_line(&mut str2).expect("ERR");
    let m: u32 = str1.trim().parse::<u32>().unwrap();
    let n: u32 = str2.trim().parse::<u32>().unwrap();

    for _ in 0..m {
        for _ in 0..n {
            print_pmmm();
        }
        println!("+");

        for _ in 0..n {
            print_vss();
        }
        print_v();
    }
    for _ in 0..n {
        print_pmmm();
    }
    print_p();
}
