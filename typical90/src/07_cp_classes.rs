use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        mut va: [usize; n],
        q: usize,
        vb_: [(usize); q],
    }
    let vb = vb_.iter().map(|x| x.0).collect::<Vec<usize>>();

    // Aをソートして二分探索する
    va.sort();

    for i in 0..q {
        let b = vb[i];
        let calc_diff = diff_factory(b);

        // 区間準備
        let mut k = 0;
        let mut l = n - 1;
        let mut j = (k + l) / 2; // NOTE: 切り捨て

        // bを超えないaの最大値を求める
        // すべてbより大きい場合は最小値を求める
        let idx: usize = loop {
            if l - k <= 1 {
                if b > va[l] {
                    break l;
                } else {
                    break k;
                }
            }

            if b < va[j] {
                // 左側に答えがある
                l = j;
                j = (k + l) / 2;
            } else {
                // 右側に答えがある
                k = j;
                j = (k + l) / 2;
            }
        };

        if idx == n - 1 {
            println!("{}", calc_diff(va[idx]));
        } else {
            println!("{}", cmp::min(calc_diff(va[idx]), calc_diff(va[idx + 1])));
        }
    }
}

fn diff_factory(b: usize) -> Box<dyn Fn(usize) -> usize> {
    Box::new(move |a| b.abs_diff(a))
}
