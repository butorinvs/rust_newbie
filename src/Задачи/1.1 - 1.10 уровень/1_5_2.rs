//Выведите в консоль все числа кратные 7 в промежутке от -100 до 100.
fn main() {
    let mut arr: Vec<i16> = Vec::new();
    for n in -100..100 {
        if n % 7 == 0 {
            arr.push(n);
        }
    }
    println!("{:?}", arr);
}
