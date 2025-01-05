use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!("{}", count(r) - count(l - 1));
}

fn pow(a: usize, b: usize) -> usize {
    (a as u64).pow(b as u32) as usize
}

// x以下の蛇数
fn count(x: usize) -> usize {
    // 10未満の場合は0を返す
    if x < 10 {
        return 0;
    }

    let digits: Vec<usize> = x.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let n = digits.len();
    let first = digits[0];

    let mut count: usize = 0;

    // n桁未満のカウント
    // Σk=1,n-2(Σl=1,9(l^k))
    for k in 1..n - 1 {
        for l in 1..=9 {
            count += pow(l, k);
        }
    }

    // n桁かつ、1~xの先頭の値未満をカウント
    for j in 1..first {
        count += pow(j, n - 1);
    }

    // n桁の場合は桁dpする
    // i桁目まで見る, is_tight
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n];

    for i in 0..n {
        // 初期値
        if i == 0 {
            dp[i][1] = 1;
            continue;
        }

        // i桁目までを考える
        let d = digits[i]; // xのi桁目
        let is_tight: usize = if dp[i - 1][1] == 1 && d < first { 1 } else { 0 }; // 前の値がtightかつ、蛇数である
        dp[i][0] = dp[i - 1][0] * first + dp[i - 1][1] * (if is_tight == 1 { d } else { first }); // smallerになる
        dp[i][1] = is_tight; // tightになりつづける
    }
    count += dp[n - 1][0] + dp[n - 1][1];

    count
}