fn read_stdio_multi_line<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v2.push(v);
    }
    v2
}

fn main() {
    let tmp: Option<i32> = read_stdio_multi_line(1)
        .concat()
        .iter()
        .fold(1i32, |acc, x| acc * x)
        .checked_rem(2);
    match tmp.unwrap() {
        0 => println!("Even"),
        _ => println!("Odd"),
    }
}
