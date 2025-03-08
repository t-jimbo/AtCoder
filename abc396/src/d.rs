use proconio::input;
use std::cmp::min;
use std::collections::HashSet;
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

    let mut stack: VecDeque<(usize, usize, HashSet<usize>)> = VecDeque::new();
    stack.push_back((0, 0, HashSet::new()));

    let mut min_w = None;

    while let Some((node, total_w, mut visited)) = stack.pop_front() {
        for &(v, w) in &graph[node] {
            if visited.contains(&v) {
                continue;
            }
            if v == n - 1 {
                if min_w.is_none() {
                    min_w = Some(total_w ^ w);
                } else {
                    min_w = Some(min(min_w.unwrap(), total_w ^ w));
                }
            }
            visited.insert(node);
            stack.push_back((v, total_w ^ w, visited.clone()));
        }
    }

    println!("{}", min_w.unwrap());
}
