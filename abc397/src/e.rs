use proconio::input;
use std::collections::VecDeque;

// TLE
fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(usize, usize); n*k - 1],
    }

    let mut graph = vec![vec![]; n * k];
    for (u, v) in uv {
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }

    let start_nodes = graph.iter().enumerate().filter_map(|(i, v)| {
        if v.len() == 1 {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<usize>>();

    let mut ok = false;
    for i in start_nodes {
        let ok_i = dfs(&graph, n, k, i);
        if ok_i {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}


fn dfs(graph: &Vec<Vec<usize>>, n: usize, k: usize, start: usize) -> bool {
    let mut visited = vec![false; n * k];

    // next node and path length
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_back((start, 0));

    let mut ok = true;
    while let Some((u, l)) = stack.pop_back() {
        visited[u] = true;
        let new_l = if l + 1 == k {
            0
        } else {
            l + 1
        };

        let mut exist_next = false;
        for &v in &graph[u] {
            if visited[v] {
                continue;
            }
            exist_next = true;
            stack.push_back((v, new_l));
        }

        if !exist_next && new_l != 0 {
            ok = false;
            break;
        }
    }

    return ok;
}