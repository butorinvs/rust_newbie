/*
Найдите сумму положительных элементов этого массива.  */
fn main() {
    let arr: [i8; 5] = [1, 2, -3, 4, -5];
    let mut summa: i8 = 0;
    for n in 0..arr.len() {
        if arr[n] > 0 {
            summa += arr[n];
        }
    }
    println!("{}", summa);
}
