fn main() {
    let v = std::io::stdin()
        .lines()
        .take(1)
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut arr= [1, 2, 3, 4, 5];
    arr[0] += v[0] as i32;
    arr[1] -= v[0] as i32;
    arr[2] *= v[0] as i32;
    arr[3] /= v[0] as i32;
    arr[4] %= v[0] as i32;
    println!("{:?}", arr);
}
