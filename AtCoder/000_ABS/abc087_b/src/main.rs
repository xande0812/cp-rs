fn read_stdio<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a: i32 = read_stdio();
    let b: i32 = read_stdio();
    let c: i32 = read_stdio();
    let x: i32 = read_stdio();
    let mut cnt: i32 = 0;

    for _a in 0..(a + 1) {
        for _b in 0..(b + 1) {
            for _c in 0..(c + 1) {
                if (_a * 500) + (_b * 100) + (_c * 50) == x {
                    cnt = cnt + 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
