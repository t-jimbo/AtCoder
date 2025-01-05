use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!("{}", count(r) - count(l - 1));
}

// どうせ差を取るので、非負整数全てカウントする
fn count(x: usize) -> usize {
    let digits: Vec<usize> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let n = digits.len();
    let first = digits[0];

    // n桁未満の場合
    // Σk=1,n-2(Σl=1,9(l^k))
    let mut smaller_digits_count: usize = 0;
    for k in 1..n - 1 {
        for l in 1..=9 {
            smaller_digits_count += (l as u32).pow(k as u32) as usize;
        }
    }

    // n桁の場合は桁dp
    // i桁, is_tight, 先頭がj
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; first + 1]; 2]; n];

    for i in 0..n {
        // i桁目までを考える
        for j in 1..=first {
            // 初期値
            if i == 0 {
                if j == first { // tight
                    dp[i][1][j] = 1;
                } else {
                    dp[i][0][j] = 1;
                }
            } else {
                if j == first { // 先頭が一致 = tightの可能性がある
                    let d = digits[i];
                    let is_tight: usize = if dp[i - 1][1][j] == 1 && d < first { 1 } else { 0 };
                    dp[i][0][j] = dp[i - 1][0][j] * j + dp[i - 1][1][j] * (if is_tight == 1 { d } else { j });
                    dp[i][1][j] = is_tight;
                } else {
                    // より小さい場合(=tightではない)
                    dp[i][0][j] = dp[i - 1][0][j] * j;
                }
            }
        }
    }

    let mut tight_digits_count = 0;
    for j in 1..=first {
        tight_digits_count += dp[n][0][j] + dp[n][1][j];
    }

    smaller_digits_count + tight_digits_count
}