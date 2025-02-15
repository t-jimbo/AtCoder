use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        _n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }

    let mut set = HashSet::new();
    let mut ans = 0;

    for (u, v) in uv {
        if u == v {
            // self loop
            ans += 1;
            continue;
        }

        let key = if u > v { (v, u) } else { (u, v) };
        if !set.contains(&key) {
            set.insert(key);
        } else {
            // multi edge
            ans += 1;
        }
    }

    println!("{}", ans);
}
