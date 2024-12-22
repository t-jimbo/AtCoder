use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    if k % 9 != 0 {
        println!("0");
        return;
    }

    // i桁で各桁の合計がjになる場合の数
    let mut dp: Vec<Vec<usize>> = vec![vec![]; k];


    for i in 0..k {
        for j in 0..k {
            if i == 0 {
                dp[i].push(if j < 9 { 1 } else { 0 });
                continue;
            }

            // 合計値 < 桁数 になることはない
            if j < i {
                dp[i].push(0);
                continue;
            }

            let mut count = 0;
            for l in start_l(j)..j {
                count = mod7(count + dp[i - 1][l]);
            }
            dp[i].push(count);
        }
    }

    let mut ans = 0;
    for i in 0..k {
        ans = mod7(ans + dp[i][k - 1])
    };
    println!("{}", ans)
}

fn start_l(j: usize) -> usize {
    if j < 9 { 0 } else { j - 9 }
}

const MOD: usize = 1000000007;

fn mod7(v: usize) -> usize {
    if v >= MOD {
        v - MOD
    } else {
        v
    }
}
