use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
    }

    let mut freq_map: Vec<HashMap<usize, usize>> = Vec::new();
    let mut kn: Vec<f64> = Vec::new();

    for _ in 0..n {
        input! {
            k: usize,
            a: [usize; k],
        }
        kn.push(k as f64);

        let mut count: HashMap<usize, usize> = HashMap::new();
        for num in a {
            *count.entry(num).or_insert(0) += 1;
        }

        freq_map.push(count);
    }

    // max9900通り
    let mut max_p = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let mut p = 0.0;

            let ki = kn[i];
            let kj = kn[j];
            // 短いループになるほうを選択
            let (s, t) = if ki < kj { (i, j) } else { (j, i) };

            for (num, &cs) in &freq_map[s] {
                if let Some(&ct) = freq_map[t].get(num) {
                    p += cs as f64 / ki * ct as f64 / kj;
                }
            }

            if max_p < p {
                max_p = p;
            }
        }
    }


    println!("{}", max_p);
}
