use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    if k % 9 != 0 {
        println!("0");
        return;
    }

    // 各桁の合計がiになるのが何通りあるかの漸化式を考える
    let mut dp: Vec<usize> = vec![];

    for i in 0..(k + 1) {
        if i == 0 {
            dp.push(1); // 合計が0のパターン
            continue;
        }
        // 合計がkになるのは、合計がk-9 ~ k-1のときに、9~1を選んだ場合
        let mut count = 0;
        for j in 1..10 {
            if i >= j {
                count = mod7(count + dp[i - j]);
            }
        }
        dp.push(count);
    };

    println!("{}", dp[k])
}

const MOD: usize = 1000000007;

fn mod7(v: usize) -> usize {
    if v >= MOD {
        v - MOD
    } else {
        v
    }
}
