use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 0;
    let mut j = 0; // 1 count
    let mut zero: Vec<usize> = Vec::new();
    for i in 0..n {
        match s[i] {
            '1' => {
                j += 1;
                if i == 0 || i == n - 1 {
                    zero.push(0);
                }
            }
            '0' => {
                if zero.len() == j {
                    zero.push(1)
                } else {
                    zero[j] += 1
                }
            }
            _ => unreachable!(),
        }
    }

    let m = j / 2;
    for i in 1..zero.len() - 1 {
        if i <= m {
            ans += zero[i] * i;
        } else {
            ans += zero[i] * (j - i);
        }
    }

    println!("{}", ans);
}
