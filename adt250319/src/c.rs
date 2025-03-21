use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, char); m],
    }

    let mut taros: HashSet<usize> = HashSet::new();

    for &(family, gender) in &ab {
        if gender != 'M' {
            println!("No");
            continue;
        }
        if taros.contains(&family) {
            println!("No");
            continue;
        }
        taros.insert(family);
        println!("Yes");
    }
}
