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
    let sum: i32 = read_stdio_multi_line(2)
        .concat()
        .iter()
        .fold(0, |acc, x| acc + x);
    let message: String = read_stdio_multi_line(1).concat().pop().unwrap();
    println!("{} {}", sum, message);
}
