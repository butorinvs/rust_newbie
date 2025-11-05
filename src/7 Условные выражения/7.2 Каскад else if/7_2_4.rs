/*
Напишите программу, которая считывает трехзначное число, а затем формирует наибольшее число из его цифр и выводит результат.

Sample Input:
123

Sample Output:
321
*/

fn main() {
    let arr = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let sotni = arr[0] / 100;
    let des = (arr[0] / 10) % 10;
    let odin = arr[0] % 10;

    if sotni >= des && des >= odin {
        if des >= odin {
            println!("{}{}{}", sotni, des, odin);
        } else {
            println!("{}{}{}", sotni, odin, des);
        }
    } else if des >= odin && des >= odin {
        if sotni >= odin {
            println!("{}{}{}", des, sotni, odin);
        } else {
            println!("{}{}{}", des, odin, sotni);
        }
    } else {
        if sotni >= des {
            println!("{}{}{}", odin, sotni, des);
        } else {
            println!("{}{}{}", odin, des, sotni);
        }
    }
}
