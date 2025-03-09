use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let counts: HashMap<usize, Vec<usize>> = a.iter().enumerate().fold(HashMap::new(), |mut acc, (i, &x)| {
        acc.entry(x).or_insert(vec![]).push(i);
        acc
    });

    let mut min = None;
    for (_, v) in counts.iter() {
        if v.len() == 1 {
            continue;
        }

        let mut min_i = None;
        for i in 0..v.len() - 1 {
            let diff = v[i + 1] - v[i];
            if min_i.is_none() || diff < min_i.unwrap() {
                min_i = Some(diff + 1);
            }
        }

        if min.is_none() || min_i.unwrap() < min.unwrap() {
            min = min_i;
        }
    }

    if min.is_none() {
        println!("-1");
    } else {
        println!("{}", min.unwrap());
    }
}
