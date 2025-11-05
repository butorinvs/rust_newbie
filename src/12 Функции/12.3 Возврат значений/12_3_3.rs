/*
В редакторе кода представлена неполная программа, которую необходимо доделать. Программа должна считать в массив arr 10 целых чисел и строку ord, задающую порядок сортировки:

    asc - сортировка по возрастанию;
    dec - сортировка по убыванию;

После вывести отсортированную коллекцию с помощью заполнителя {:?}.

Sample Input:
8
9
-1
0
10
-8
-2
7
-7
-3
asc

Sample Output:
[-8, -7, -3, -2, -1, 0, 7, 8, 9, 10]
*/
fn get_input() -> Vec<i32> {
    let arr = std::io::stdin()
        .lines()
        .take(10)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    arr
}

fn sort_input() -> String {
    let mut str1 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    str1.trim().to_string()
}

fn sort(mut arr: Vec<i32>, ord: &str) -> Vec<i32> {
    if ord == "asc" {
        arr.sort();
    } else if ord == "dec" {
        arr.sort_by(|a, b| b.cmp(a));
    }
    arr
}

fn main() {
    let number = get_input();
    let string = sort_input();
    let array = sort(number, &string);
    println!("{:?}", array);
}
