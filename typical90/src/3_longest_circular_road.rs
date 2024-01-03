use proconio::input;

type Graph = Vec<Vec<usize>>;

// FIXME: どこが間違ってるのかよくわからない
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
    let dist = dfs(&graph, 0);
    let (max_i, _) = max(dist);

    let rs = dfs(&graph, max_i);
    let (_, r) = max(rs);
    println!("{}", r + 1);
}

// depth first searchで最も遠いノードと距離を求める
fn dfs(graph: &Graph, s: usize) -> Vec<i32> {
    let mut stack: Vec<usize> = vec![s];
    let mut dist = vec![-1; graph.len()];
    dist[s] = 0;

    while stack.len() > 0 {
        let v = stack.pop().unwrap();
        for i in 0..graph[v].len() {
            if graph[v][i] == 1 && dist[i] == -1 {
                stack.push(i);
                dist[i] = dist[v] + 1;
            }
        }
    }

    dist
}

// 最小値とそのindexを返す(最小値は-1とする)
fn max(arr: Vec<i32>) -> (usize, i32) {
    arr.iter().enumerate().fold(
        (0, -1),
        |(max_i, max), (i, v)| if v > &max { (i, *v) } else { (max_i, max) },
    )
}
