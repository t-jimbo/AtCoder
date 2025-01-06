use proconio::{input, marker::Chars};
use std::cmp::min;
use std::collections::{HashMap, VecDeque};

type Position = (usize, usize);
type Graph = Vec<Vec<Vec<Position>>>;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);

    let mut g1: Graph = vec![vec![vec![]; w]; h]; // 0, 0で横に移動
    let mut g2: Graph = vec![vec![vec![]; w]; h]; // 0, 0で縦に移動

    for i in 0..h {
        for j in 0..w {
            // startとgoalを記録しておく
            if s[i][j] == 'S' { start = (i, j); }
            if s[i][j] == 'G' { goal = (i, j); }

            // それぞれの方向に移動可能かどうか
            let is_left_enabled = j != 0 && s[i][j - 1] != '#';
            let is_right_enabled = j != w - 1 && s[i][j + 1] != '#';
            let is_up_enabled = i != 0 && s[i - 1][j] != '#';
            let is_down_enabled = i != h - 1 && s[i + 1][j] != '#';

            if (i + j) % 2 == 0 {
                // 1は横移動, 2は縦移動のみ可能
                if is_left_enabled { g1[i][j].push((i, j - 1)); }
                if is_right_enabled { g1[i][j].push((i, j + 1)); }
                if is_up_enabled { g2[i][j].push((i - 1, j)); }
                if is_down_enabled { g2[i][j].push((i + 1, j)); }
            } else {
                // 1は縦移動、2は横移動のみ可能
                if is_left_enabled { g2[i][j].push((i, j - 1)); }
                if is_right_enabled { g2[i][j].push((i, j + 1)); }
                if is_up_enabled { g1[i][j].push((i - 1, j)); }
                if is_down_enabled { g1[i][j].push((i + 1, j)); }
            }
        }
    }

    let res1 = bfs(g1, start, goal);
    let res2 = bfs(g2, start, goal);

    println!("{}", min(res1, res2));
}

fn bfs(g: Graph, start: Position, goal: Position) -> isize {
    let mut queue: VecDeque<Position> = VecDeque::new();
    let mut visited: HashMap<Position, isize> = HashMap::new();
    queue.push_back(start);
    visited.insert(start, 0);

    loop {
        let p = queue.pop_front();
        match p {
            Some(p) => {
                let step = visited.get(&p).unwrap() + 1;
                for next in g[p.0][p.1].iter() {
                    if !visited.contains_key(next) {
                        queue.push_back(*next);
                        visited.insert(*next, step);
                    }
                }
            }
            None => { break; }
        }
    };

    *visited.get(&goal).unwrap_or(&-1)
}
