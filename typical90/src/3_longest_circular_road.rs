use proconio::input;

type Graph = Vec<Vec<usize>>;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }

    // 双方向グラフを作成
    let mut graph: Graph = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        let (a, b) = ab[i];
        graph[a - 1][b - 1] = 1;
        graph[b - 1][a - 1] = 1;
    }

    // 最も遠い点P
    let (_, p) = dfs(&graph, 0);
    println!("P: {}", p);
    let (r, _) = dfs(&graph, p);
    println!("{}", r + 1);
}

// depth first searchで最も遠いノードと距離を求める
fn dfs(graph: &Graph, s: usize) -> (usize, usize) {
    let mut stack: Vec<usize> = vec![s];
    let mut dist = 0;
    let mut prev = 0;

    while stack.len() > 0 {
        let v = stack.pop().unwrap();
        for i in 0..graph[v].len() {
            // FIXME
            if i != prev && graph[v][i] == 1 {
                stack.push(i);
                dist += 1;
            }
        }
        prev = v;
    }

    (dist, prev)
}
