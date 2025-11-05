fn main() {
    let mut input = String::new();
    let _result: usize = std::io::stdin().read_line(&mut input).expect("");
    let input = input.trim();
    let arr: [i32; 5] = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .take(5)
        .collect::<Vec<i32>>()
        .try_into()
        .expect("Need exactly 5 digits");
    let mut sum_pos1: i32 = 0;
    let mut sum_pos3: i32 = 0;
    for i in 0..arr.len() / 2 {
        sum_pos1 += arr[i];
        sum_pos3 += arr[i + 3];
    }
    if sum_pos1 == sum_pos3 {
        println!("YES");
    } else {
        println!("NO");
    }
}
