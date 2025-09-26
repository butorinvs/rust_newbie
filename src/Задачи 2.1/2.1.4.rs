/* Дан массив со строками:
let arr = ["123", "456", "789"];

Выведите в консоль первые символы каждого элемента этого массива.  */
fn main() {
    let arr = ["523", "456", "789"];
    let mut n = 0;
    while n < arr.len() {
        let txt: String = (&arr[n]).to_string();
        let first = txt.chars().next().unwrap();
        println!("{:?}", first);
        n += 1;
    }
}
/*fn main() {
    let arr = ["523", "456", "789"];
    for s in &arr {
        if let Some(first) = s.chars().next() {
            println!("{:?}", first);
        }
    }
}
 */
