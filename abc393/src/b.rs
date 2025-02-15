use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let mut ans = 0;

    for i in 0..(n - 2) {
        if s[i] != 'A' {
            continue;
        }

        let max_d = (n - i - 1) / 2;
        for d in 1..=max_d {
            if s[i + d] == 'B' && s[i + 2 * d] == 'C' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
