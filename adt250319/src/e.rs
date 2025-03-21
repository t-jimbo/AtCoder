use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
    }

    let mut s: BTreeMap<usize, usize> = BTreeMap::new();

    for _ in 0..q {
        input! {
            cmd: usize,
        }
        match cmd {
            1 => {
                input! {
                    x: usize,
                }
                *s.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                    c: usize,
                }
                if let Some(v) = s.get_mut(&x) {
                    *v = v.saturating_sub(c);
                    if *v == 0 {
                        s.remove(&x);
                    }
                }
            }
            3 => {
                if s.is_empty() {
                    panic!("s is empty");
                }

                if let Some(&min) = s.keys().next() {
                    if let Some(&max) = s.keys().next_back() {
                        println!("{}", max - min);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
