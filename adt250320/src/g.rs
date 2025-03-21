use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
    }

    let mut time = 0;
    // t -> count
    let mut pot: BTreeMap<usize, usize> = BTreeMap::new();

    for _ in 0..q {
        input! {
            cmd: usize,
        }
        match cmd {
            1 => {
                *pot.entry(time).or_insert(0) += 1;
            }
            2 => {
                input! {
                    t: usize,
                }
                time += t;
            }
            3 => {
                input! {
                    h: usize,
                }
                if time < h {
                    println!("0");
                    continue;
                }
                let max_time = time - h;
                let mut count = 0;
                let mut cut = vec![];

                for (key, value) in pot.range(..=max_time) {
                    cut.push(*key);
                    count += value;
                }

                for key in cut {
                    pot.remove(&key);
                }
                println!("{}", count);
            }
            _ => unreachable!(),
        }
    }
}
