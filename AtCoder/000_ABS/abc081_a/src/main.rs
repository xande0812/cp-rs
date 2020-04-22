fn read_stdio<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let tmp: u32 = read_stdio::<String>()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |acc, x| acc + x);
    println!("{:?}", tmp);
}
