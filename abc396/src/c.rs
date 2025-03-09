use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [isize; n],
        mut w: [isize; m],
    }

    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));

    let mut dp = vec![0; n + 1];

    for i in 0..n {
        let diff = if i < m {
            max(max(0, b[i]), b[i] + w[i])
        } else {
            max(0, b[i])
        };

        if diff == 0 {
            println!("{}", dp[i]);
            return;
        } else {
            dp[i + 1] = dp[i] + diff;
        }
    }
    println!("{}", dp[n]);
}
