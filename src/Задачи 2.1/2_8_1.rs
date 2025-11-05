/* Дан массив со строками:
let arr: [&str; 3] = ["ab", "cd", "ef"];

Получите массив символов этих строк:
['a', 'b', 'c', 'd', 'e', 'f'] */
fn main() {
    let arr: [&str; 3] = ["ab", "cd", "ef"];
    let iter: Vec<char> = arr.iter().flat_map(|&s| s.chars()).collect();
    println!("{:?}", iter);
    //let len = iter.len() - 1;
    //println!("{}", iter[len]);
}
