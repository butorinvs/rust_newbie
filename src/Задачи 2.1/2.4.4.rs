/*
Зполните массив от 10 до 1 */
fn main() {
    let mut arr: [u8; 10] = [0; 10];
    let mut n: usize = 10;
    for i in 0..arr.len() {
        arr[i] = n as u8;
        n -= 1;
    }
    println!("{:?}", arr);
}
