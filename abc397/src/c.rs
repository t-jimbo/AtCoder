use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut left: HashMap<usize, usize> = HashMap::new();
    let mut right: HashMap<usize, usize> = HashMap::new();
    for &i in &a {
        *right.entry(i).or_insert(0) += 1;
    }

    let mut max = 0;
    for i in 0..n - 1 {
        // count up or add
        *left.entry(a[i]).or_insert(0) += 1;
        // count down or remove
        if let Some(count) = right.get_mut(&a[i]) {
            if *count == 1 {
                right.remove(&a[i]);
            } else {
                *count -= 1;
            }
        }

        let unique_sum = left.len() + right.len();
        max = max.max(unique_sum);
    }

    println!("{}", max);
}
