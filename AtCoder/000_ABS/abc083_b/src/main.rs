fn read_stdio_to_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let input: Vec<u32> = read_stdio_to_vec();
    println!(
        "{:?}",
        (1..input[0] + 1)
            .filter(|x| {
                let base: u32 = x.to_string().chars().map(|e| e.to_digit(10).unwrap()).sum();
                input[1] <= base && base <= input[2]
            })
            .sum::<u32>()
    );
}
