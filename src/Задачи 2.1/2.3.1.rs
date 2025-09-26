// Найдите сумму квадратов элементов этого массива.
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut summ: i32 = 0;
    for n in 0..arr.len() {
        summ += arr[n] * arr[n];
    }
    println!("{:?}", summ);
}
