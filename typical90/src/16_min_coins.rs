use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let min = solve2(b, c, a, n);

    std::println!("{}", min)
}

fn solve2(a: usize, b: usize, c: usize, n: usize) -> usize {
    // Aを最大で何枚使えるかどうか
    let max_q_a = cmp::min(n / a, 9999);
    let mut min = 0;

    for q_a in 0..(max_q_a + 1) {
        let rest = n - q_a * a;
        match solve_bc(b, c, rest) {
            Some(q_bc) => {
                let q = q_bc + q_a;
                if min == 0 || min > q {
                    min = q;
                };
            }
            None => {
                continue;
            }
        }
    };

    if min == 0 {
        panic!("cannot be solved")
    }

    min
}

fn solve_bc(b: usize, c: usize, n: usize) -> Option<usize> {
    // Bを最大で何枚使えるかどうか
    let max_q_b = cmp::min(n / b, 9999);
    let mut min: Option<usize> = None;

    for q_b in 0..(max_q_b + 1) {
        let rest = n - q_b * b;
        match solve_c(c, rest) {
            Some(q_c) => {
                let q = q_c + q_b;
                match min {
                    Some(m) => { if m > q { min = Some(q) } }
                    None => { min = Some(q) }
                }
            }
            None => {
                continue;
            }
        }
    };

    min
}

fn solve_c(c: usize, n: usize) -> Option<usize> {
    // cで割る
    let r = n % c;
    let q = n / c;
    // 割り切れればOK
    if r == 0 && q <= 9999 {
        return Some(q);
    }

    None
}
