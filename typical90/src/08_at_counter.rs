use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let a: Vec<char> = "atcoder".chars().collect();

    // i文字目まで使って、`atcoder`のj文字目までを抽出する方法が何通りあるかの漸化式を考える
    // NOTE: 再帰にするとi, jの計算結果をキャッシュしない限り都度計算するのでO(2^n)になってしまう
    let mut dp: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for i in 0..(n + 1) {
        for j in 0..(a.len() + 1) {
            if j == 0 {
                dp[i].push(1);
                continue;
            }
            if i == 0 {
                dp[i].push(0);
                continue;
            }

            let prev = dp[i - 1][j];
            if s[i - 1] != a[j - 1] {
                dp[i].push(prev);
                continue;
            }

            let prev_left = dp[i - 1][j - 1];
            dp[i].push(mod7(prev + prev_left));
        }
    }

    println!("{}", dp[n][a.len()])
}

fn mod7(v: usize) -> usize {
    if v >= MOD {
        v - MOD
    } else {
        v
    }
}
