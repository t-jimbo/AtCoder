use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut pigeon_to_ref: Vec<usize> = (0..n).collect();
    let mut ref_to_nest: Vec<usize> = (0..n).collect();
    let mut nest_to_ref: Vec<usize> = (0..n).collect();

    for _ in 0..q {
        input! {
            c: usize,
        }

        match c {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                // move pigeon_a to nest_b
                let r = nest_to_ref[b - 1];
                pigeon_to_ref[a - 1] = r;
            }
            2 => {
                input! {
                    a: usize,
                    b: usize,
                }
                // swap nest a, b
                let i = nest_to_ref[a - 1];
                let j = nest_to_ref[b - 1];
                ref_to_nest.swap(i, j);
                nest_to_ref.swap(a - 1, b - 1);
            }
            3 => {
                input! {
                    a: usize,
                }
                println!("{}", ref_to_nest[pigeon_to_ref[a - 1]] + 1);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
