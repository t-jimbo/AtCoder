use proconio::input;

fn main() {
    input! {
        n: usize, // 生徒数
        cp: [(usize, usize); n], // クラスと得点
        q: usize, // question数
        lr: [(usize, usize); q], // 学籍番号の両端
    }

    // #0~nまでのclass1, 2の得点合計値
    let mut sums: Vec<(usize, usize)> = Vec::with_capacity(n + 1);
    sums.push((0, 0));

    // sumsを前処理
    for i in 0..n {
        let (c, p) = cp[i];
        let (prev_1, prev_2) = if i == 0 { (0, 0) } else { sums[i] };
        if c == 1 {
            sums.push((prev_1 + p, prev_2));
        } else {
            sums.push((prev_1, prev_2 + p));
        }
    }

    // SUM(lr) = SUM(r) - SUM(l)に分解して考える
    for i in 0..q {
        let (l, r) = lr[i];
        let (sum_l1, sum_l2) = sums[l - 1];
        let (sum_r1, sum_r2) = sums[r];

        std::println!("{} {}", sum_r1 - sum_l1, sum_r2 - sum_l2);
    }
}
