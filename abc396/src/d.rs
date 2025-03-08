use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(usize, usize, usize); m],
    }

    let mut graph = vec![Vec::new(); n];

    for (u, v, w) in uvw {
        graph[u - 1].push((v - 1, w));
        graph[v - 1].push((u - 1, w));
    }

    let mut xor: Vec<Option<usize>> = vec![None; n];

    let mut queue = VecDeque::new();
    queue.push_back(0);
    xor[0] = Some(0);

    while let Some(u) = queue.pop_front() {
        for &(v, w) in &graph[u] {
            if xor[v].is_none() {
                xor[v] = Some(xor[u].unwrap() ^ w);
                queue.push_back(v);
            } else {
                let xor_v = xor[v].unwrap();
                let new = min(xor_v, xor[u].unwrap() ^ w);
                if new != xor_v {
                    xor[v] = Some(new);
                    queue.push_back(v);
                }
            }
        }
    }

    println!("{:?}", xor);
}