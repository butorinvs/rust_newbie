/*
Найдите сумму элементов этого массива.
 */
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut summa: i32 = 0;
    for n in 0..arr.len() {
        summa += arr[n];
    }
    print!("{:?}", summa);
}
