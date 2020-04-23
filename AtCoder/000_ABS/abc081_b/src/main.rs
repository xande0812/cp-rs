fn read_stdio<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn gcd_vec(vec: &Vec<u64>) -> u64 {
    vec.iter().fold(vec[0], |a, b| gcd(a, *b))
}

fn main() {
    let _: i32 = read_stdio();
    let mut nums: Vec<u64> = read_vec();
    let mut cnt: i32 = 0;
    loop {
        if let Some(1) = gcd_vec(&nums).checked_rem(2) {
            break;
        }
        nums = nums.iter().map(|e| e / 2).collect();
        cnt = cnt + 1;
    }
    println!("{:?}", cnt);
}
