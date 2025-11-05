//Выведите в консоль все четные числа из промежутка от 2 до 100.
fn main() {
    let mut arr: Vec<u16> = Vec::new();
    for n in 2..100 {
        if n % 2 == 0 {
            arr.push(n);
        }
    }
    println!("{:?}", arr);
}
