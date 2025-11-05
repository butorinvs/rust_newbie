/*
Увеличьте каждый элемент этого массива в два раза: */
fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    for n in 0..arr.len() {
        arr[n] *= 2;
    }
    println!("{:?}", arr);
}
