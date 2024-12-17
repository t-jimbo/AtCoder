use proconio::input;

fn main() {
    input! {
        w: usize,
        d: usize,
        h: usize,
    }

    // 最大公約数の立方体になるようにすればよさそう
    let _gcd = calc_gcd(w, d);
    let gcd = calc_gcd(h, _gcd);

    let ans = w / gcd + d / gcd + h / gcd - 3;

    std::println!("{}", ans);
}

// 2つの数の最大公約数を互除法で求める
fn calc_gcd(a: usize, b: usize) -> usize {
    let r = a % b;

    if r == 0 {
        return b;
    }

    calc_gcd(b, r)
}