/*
В редакторе кода представлен кортеж значений. Деструктуризируйте кортеж в отдельные переменные и выведите их, как показано в примере.

Sample Input:

Sample Output:
До, деструктуризации, 3, 2, 1, 0, пуск!
*/

fn main() {
    let tup = ("До", "деструктуризации", 3, 2, 1, 0, "пуск!");
    let (str1, str2, chislo1, chislo2, chislo3, chislo4, str3) = tup;
    println!("{str1}, {str2}, {chislo1}, {chislo2}, {chislo3}, {chislo4}, {str3}");
}
