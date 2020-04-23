fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

// 参考
// https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-3-%E5%95%8F-abc-081-b---shift-only-200-%E7%82%B9
fn main() {
    let _: Vec<u32> = read_vec();
    let nums: Vec<u32> = read_vec();
    println!("{}", nums.iter().map(|e| e.trailing_zeros()).min().unwrap());
}
