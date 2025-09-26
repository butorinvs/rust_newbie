/* Дан массив с датой:
let arr: [&str; 3] = ["2025", "12", "31"];

Из элементов этого массива соберите дату в следующем формате:
"31-12-2025" */
fn main() {
    let arr: [&str; 3] = ["2025", "12", "31"];
    let mut txt: String = String::new();
    for n in (0..arr.len()).rev() {
        txt.push_str(arr[n]);
        if n == 0 {
            break;
        }
        txt.push('-');
    }
    println!("{}", txt);
}
