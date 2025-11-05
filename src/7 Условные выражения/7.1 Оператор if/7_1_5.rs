/*

*/

fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    if v[0] < 24.0 {
        println!(
            "Температура {:.1}°C ниже нормы 24.0°C, включаю отопление",
            v[0]
        );
    } else if v[0] > 28.0 {
        println!(
            "Температура {:.1}°C выше нормы 28.0°C, отключаю отопление",
            v[0]
        );
    } else if v[0] >= 24.0 || v[0] <= 28.0 {
        println!("Температура {:.1}°C в пределах нормы", v[0]);
    }
}
