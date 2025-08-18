/*

*/

fn main() {
    let arr = [
        -2.5, 4.2, 9.1, 22.5, 30.0, 1445.123, 1000000.0, 0.001, 0.5, -0.127,
    ];
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] < 0 {
        println!("Отрицательный индекс приводит к панике");
    } else if v[0] >= arr.len() as i32 {
        println!("Попытка выхода за пределы массива");
    } else {
        let index: usize = v[0] as usize;
        println!("Элемент с индексом {} равен {:.3}", index, arr[index]);
    }
}
