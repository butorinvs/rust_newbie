fn get_primary_sc() {
    let mut str1 = String::new();
    std::io::stdin().read_line(&mut str1).expect("ERR");
    let index: u8 = str1.trim().parse::<u8>().unwrap();

    let arr = [
        0, 7, 14, 20, 27, 34, 40, 43, 46, 48, 51, 54, 56, 59, 62, 64, 67, 70, 72, 75, 78, 80, 83,
        85, 88, 90, 93, 95, 98, 100,
    ];
    print_test_sc(arr, index);
}

fn print_test_sc(ar: [i32; 30], ind: u8) {
    let ind1: usize = ind as usize;
    println!("{}", ar[ind1]);
}

fn main() {
    get_primary_sc();
}
