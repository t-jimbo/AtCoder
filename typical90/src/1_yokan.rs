fn main() {
    let input = parse_input::<usize>(3);
    let l = input[0][1];
    let k = input[1][0];
    let a = &input[2];
    let a = &[a.to_owned(), vec![l]].concat();
    let ans = solve(0, l, a, k);
    println!("{}", ans);
}

// stdin
fn parse_input<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v2.push(v);
    }
    v2
}

// 与えられた条件を満たす切り方があるかどうか
fn is_party(m: usize, a: &[usize], k: usize) -> bool {
    let mut count = 0;
    let mut cut_len = 0;
    for i in 0..a.len() {
        if i == 0 {
            cut_len = a[i];
        } else {
            cut_len += a[i] - a[i - 1];
        }

        if count <= k && cut_len >= m {
            count += 1;
            cut_len = 0;
        }
    }

    if count > k {
        return true;
    }

    cut_len >= m && count == k
}

// 最小値の最大値を求める -> 答えを二部探索する
fn solve(left: usize, right: usize, a: &[usize], k: usize) -> usize {
    if left == right {
        return left;
    }
    if left + 1 == right {
        if is_party(right, a, k) {
            return right;
        } else {
            return left;
        }
    }

    let mid = (left + right) / 2;

    if is_party(mid, a, k) {
        solve(mid, right, a, k)
    } else {
        solve(left, mid, a, k)
    }
}
