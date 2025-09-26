/*
Реализация функции возведения числа в степень может быть выполнена несколькими способами.
Одним из наиболее известных и эффективных алгоритмов является алгоритм быстрого возведения в степень (exponentiation by squaring).
Этот метод работает для целых и вещественных чисел. Алгоритм быстрого возведения в степень можно формализовать в следующем виде:

an={1if n==01a−nif n<0(an2)2if n>0 и n четное(an−12)2⋅aif n>0 и n нечетное
an=⎩
⎨

⎧​1a−n1​(a2n​)2(a2n−1​)2⋅a​if n==0if n<0if n>0 и n четноеif n>0 и n нечетное​

В редакторе кода представлена неполная программа, которую необходимо доделать.
Программа должна считывать вещественное число a и целое число n, а затем выводить (до 3 знаков) результат anan.

Вычисление возведения в степень реализуйте в функции pow() !

Sample Input:
1
0

Sample Output:
1.000
 */
fn pow(base: f64, exp: i32) -> f64 {
    if exp == 0 {
        return 1.0;
    } else if exp < 0 {
        return 1.0 / pow(base, -exp);
    }
    let half = pow(base, exp / 2);
    if exp % 2 == 0 {
        half * half
    } else {
        half * half * base
    }
}

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();

    std::io::stdin().read_line(&mut str1).expect("ERR");
    std::io::stdin().read_line(&mut str2).expect("ERR");

    let base: f64 = str1.trim().parse::<f64>().unwrap();
    let exp: i32 = str2.trim().parse::<i32>().unwrap();
    let result = pow(base, exp);
    println!("{result:.3}");
}
