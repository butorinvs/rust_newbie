/*
Напишите программу, которая считывает название города и два целых числа  и определяет,
разрешен ли им проход в город или нет, как показано в примере.

Sample Input:
Четный
1
2

Sample Output:
1 в город Четный вход запрещен
2 в город Четный вход разрешен
*/

fn main() {
    let string = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    let arr = std::io::stdin()
        .lines()
        .take(2)
        .map(|x| x.unwrap().trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if string[0] == "Четный" {
        if (arr[0] % 2) != 0 {
            println!("{} в город {} вход запрещен", arr[0], string[0]);
        } else if (arr[0] % 2) == 0 {
            println!("{} в город {} вход разрешен", arr[0], string[0]);
        }
        if (arr[1] % 2) != 0 {
            println!("{} в город {} вход запрещен", arr[1], string[0]);
        } else if (arr[1] % 2) == 0 {
            println!("{} в город {} вход разрешен", arr[1], string[0]);
        }
    }
    if string[0] == "Нечетный" {
        if (arr[0] % 2) == 0 {
            println!("{} в город {} вход запрещен", arr[0], string[0]);
        } else if (arr[0] % 2) != 0 {
            println!("{} в город {} вход разрешен", arr[0], string[0]);
        }
        if (arr[1] % 2) == 0 {
            println!("{} в город {} вход запрещен", arr[1], string[0]);
        } else if (arr[1] % 2) != 0 {
            println!("{} в город {} вход разрешен", arr[1], string[0]);
        }
    }
}
