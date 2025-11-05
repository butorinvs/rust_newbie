/*
Напишите программу, которая считывает строку, два целых числа start и end, а затем выводит считанную строку с диапазоном start..end.

Sample Input:
Я люблю учиться!
0
10

Sample Output:
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
Я люблю учиться!
*/
fn main() {
    let string = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    let v = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    for _0 in v[0]..v[1] {
        println!("{}", string[0]);
    }
}
