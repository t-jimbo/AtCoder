fn main() {
    // println!("{}", (0 + 34) / 2)
    println!("{}", solve(0, 34, &[8, 13, 26, 34], 1));
}

// 与えられた条件を満たす切り方があるかどうか
fn is_party(m: usize, a: &[usize], n: usize) -> bool {
    let mut count = 0;
    let mut cut_len = 0;
    for i in 0..a.len() {
        if i == 0 {
            cut_len = a[i];
        } else {
            cut_len += a[i] - a[i - 1];
        }

        if count < n && cut_len >= m {
            count += 1;
            cut_len = 0;
        }
    }

    cut_len >= m && count == n
}

// 最小値の最大値を求める -> 答えを二部探索する
fn solve(left: usize, right: usize, a: &[usize], n: usize) -> usize {
    if left == right {
        return left;
    }
    if left + 1 == right {
        if is_party(right, a, n) {
            return right;
        } else {
            return left;
        }
    }

    let mid = (left + right) / 2;

    if is_party(mid, a, n) {
        solve(mid, right, a, n)
    } else {
        solve(left, mid, a, n)
    }
}
