/*
Напишите программу, которая считывает четыре целых числа (x, a, b, и c) и выводит,
является ли x делителем чисел a, b и c одновременно, как показано в примерах.

    Если является вывести: {x} является делителем чисел {a}, {b}, {c}.
    Иначе вывести: {x} не является делителем всех чисел.

*/
fn main() {
    let v = std::io::stdin()
        .lines()
        .take(4)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    if v[1] % v[0] == 0 && v[02] % v[0] == 0 && v[3] % v[0] == 0 {
        println!(
            "{} является делителем чисел {}, {}, {}",
            v[0], v[1], v[2], v[3]
        );
    } else {
        println!("{} не является делителем всех чисел", v[0]);
    }
}

#[test]
fn divides_all_numbers() {
    let input = "2\n4\n6\n8\n";
    let expected = "2 является делителем чисел 4, 6, 8\n";

    let mut cmd = assert_cmd::Command::cargo_bin("8_1_2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(expected);
}

#[test]
fn not_divides_all_numbers() {
    let input = "3\n9\n10\n12\n";
    let expected = "3 не является делителем всех чисел\n";

    let mut cmd = assert_cmd::Command::cargo_bin("8_1_2").unwrap();
    cmd.write_stdin(input).assert().success().stdout(expected);
}
