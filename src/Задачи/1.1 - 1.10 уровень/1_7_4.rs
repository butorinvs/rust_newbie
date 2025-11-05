//
fn main() {
    let txt1: &str = "123";
    let txt2: &str = "456";
    let txt3: &str = "789";
    let digit1: u32 = txt1.parse().expect("Not a digit");
    let digit2: u32 = txt2.parse().expect("Not a digit");
    let digit3: u32 = txt3.parse().expect("Not a digit");
    let combined: u32 = digit1 + digit2 + digit3;

    println!("{}", combined);
}
