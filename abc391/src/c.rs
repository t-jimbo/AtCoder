use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut pigeon_hole: Vec<usize> = (0..n).collect();
    let mut hole_count: Vec<usize> = vec![1; n];
    let mut count = 0;

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
                pigeon_hole[p - 1] = h - 1; // update pigeon hole

                // update hole count
                hole_count[current] -= 1;
                if hole_count[current] == 1 {
                    count -= 1;
                }
                hole_count[h - 1] += 1;
                if hole_count[h - 1] == 2 {
                    count += 1;
                }
            }
            2 => {
                println!("{}", count);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
