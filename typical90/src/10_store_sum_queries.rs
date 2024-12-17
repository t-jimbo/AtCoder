use proconio::input;

fn main() {
    input! {
        n: usize, // 生徒数
        cp: [(usize, usize); n], // クラスと得点
        q: usize, // question数
        lr: [(usize, usize); q], // 学籍番号の両端
    }

    // questionを処理していく
    for i in 0..q {
        let mut sum1 = 0;
        let mut sum2 = 0;
        let (l, r) = lr[i];

        // 得点を合計
        for j in (l - 1)..r {
            let (c, p) = cp[j];
            if c == 1 {
                sum1 += p;
            } else {
                sum2 += p;
            }
        }

        std::println!("{} {}", sum1, sum2);
    }
}
