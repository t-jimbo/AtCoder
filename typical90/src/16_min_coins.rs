use proconio::input;

fn main() {
    input! {
        n: usize,
        mut coins: [usize; 3],
    }

    // a, b, cを使って最小の枚数でnを満たす
    coins.sort();
    let a = coins[0];
    let b = coins[1];
    let c = coins[2];


    let (n_a, n_b, n_c) = solve(a, b, c, n);

    std::println!("{}, {}, {}", a, b, c);
    std::println!("{}, {}, {}", n_a, n_b, n_c);
    std::println!("{}", n_a + n_b + n_c)
}

fn solve(a: usize, b: usize, c: usize, n: usize) -> (usize, usize, usize) {
    // cで割れるだけ割る
    let r_c = n % c;
    let n_c = n / c;
    if r_c == 0 {
        return (0, 0, n_c);
    }

    div_abc(r_c, b, c, n_c, a)
}

fn div_ab(p: usize, a: usize, b: usize, n_b: usize) -> Option<(usize, usize)> {
    // aで割る
    let r = p % a;
    // 割り切れればOK
    if r == 0 {
        return Some((p / a, n_b));
    }

    // b, aの組み合わせを変えても割り切れなかった場合
    if n_b == 0 {
        return None;
    }

    // 割り切れなければbの商を-1して再帰
    div_ab(p + b, a, b, n_b - 1)
}

fn div_abc(p: usize, b: usize, c: usize, n_c: usize, a: usize) -> (usize, usize, usize) {
    // bで割れるだけ割る
    let r = p % b;
    let n_b = p / b;
    if r == 0 {
        // 割れたらOK
        return (0, n_b, n_c);
    }

    // あまったら、aで割れるかtry
    let n_a_option = div_ab(r, a, b, n_b);
    match n_a_option {
        // 割れればOK
        Some((n_a, n_b)) => (n_a, n_b, n_c),
        None => {
            // どこかで必ず割り切れるはず。。
            if n_c == 0 {
                panic!("n_c == 0")
            }

            // いけなかったらcの商を-1して再帰
            div_abc(p + c, b, c, n_c - 1, a)
        }
    }
}
