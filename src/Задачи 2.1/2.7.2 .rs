/*
 Дан массив с числами:
let arr: [u8; 5] = [1, 2, 3, 4, 5];

Слейте элементы этого массива в строку:
"12345"*/
fn main() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let txt: String = arr
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{:?}", txt);
}
