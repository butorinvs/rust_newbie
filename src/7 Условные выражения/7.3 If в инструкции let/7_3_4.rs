/*
В редакторе кода представлен массив значений. Дополните код и считайте два индекса (usize),
 а затем выведите, являются ли считанные индексы индексами минимального или максимального
  элемента в массиве соответственно, как показано в примерах.
*/

fn main() {
    let mut input_min = String::new();
    let mut input_max = String::new();
    std::io::stdin().read_line(&mut input_min).unwrap();
    std::io::stdin().read_line(&mut input_max).unwrap();
    let min = input_min.trim().parse::<usize>().unwrap();
    let max = input_max.trim().parse::<usize>().unwrap();
    let array = [3, 1, 0, -5, -1, 300, 4, 2];
    let max_arr = *array.iter().max().unwrap();
    let min_arr = *array.iter().min().unwrap();
    if min_arr == array[min] {
        println!("Считанный мин.индекс корректный");
    } else {
        println!("Считанный мин.индекс некорректный");
    }
    if max_arr == array[max] {
        println!("Считанный макс.индекс корректный");
    } else {
        println!("Считанный макс.индекс некорректный");
    }
}
