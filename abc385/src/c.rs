use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    // 考えられる間隔についてループ回す
    let mut max = 1;
    for m in 1..n {
        // 開始位置sに関するループ
        for s in 0..(n - m) {
            // 何回連続できるかをカウント
            let mut count = 1;
            for j in (s + 1..n).step_by(m) {
                if h[j] != h[s] {
                    break;
                }
                count += 1;
            };

            // 最大値更新
            if max < count {
                max = count;
            }
        }
    };
    println!("{}", max)
}