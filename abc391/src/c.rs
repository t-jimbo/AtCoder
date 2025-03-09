use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut pigeon_hole: Vec<usize> = (0..n).collect();
    let mut hole_count: Vec<usize> = vec![1; n];

    for _ in 0..q {
        input! {
            c: usize,
        }

        match c {
            1 => {
                input! {
                    p: usize,
                    h: usize,
                }
                let current = pigeon_hole[p - 1];
                hole_count[current] -= 1;
                hole_count[h - 1] += 1;
                pigeon_hole[p - 1] = h - 1;
            }
            2 => {
                let mut count = 0;
                for i in 0..n {
                    if hole_count[i] > 1 {
                        count += 1;
                    }
                }
                println!("{}", count);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
