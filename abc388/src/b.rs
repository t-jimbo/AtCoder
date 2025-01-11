use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        tl: [(usize, usize); n],
    }

    // 愚直にやってみる
    for k in 1..=d {
        let mut max_w = 0;
        for (t, l) in &tl {
            let w = t * (l + k);
            if w > max_w {
                max_w = w;
            }
        }
        println!("{}", max_w);
    }
}
