use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        a: [usize; n], // sorted
    }

    let mut count = 0;

    // 上のモチ
    for i in 0..n {
        // できない
        let upper_mochi = a[i];
        if upper_mochi * 2 > a[n - 1] { break; }

        let mut l = i + 1;
        let mut r = n - 1;
        loop {
            let mid = (l + r) / 2;
            if l == r {
                break;
            }

            if is_kagamimochable(upper_mochi, a[mid]) {
                r = max(mid - 1, l);
                if l + 1 == r {}
            } else {
                l = min(mid + 1, r);
            }
        }

        let max_ng = if is_kagamimochable(upper_mochi, a[l]) { l } else { l + 1 };
        count += n - max_ng;
    }

    println!("{}", count);
}

fn is_kagamimochable(a: usize, b: usize) -> bool {
    a * 2 <= b
}
