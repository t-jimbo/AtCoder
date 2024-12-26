use proconio::input;
use std::collections::HashMap;


fn main() {
    input! {
        n: usize,
        m: usize,
        mut sx: usize, // サンタの座標
        mut sy: usize,
        xy: [(usize, usize); n], // 家
        dc: [(char, usize); m], // cmd
    }

    let mut x_houses: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut y_houses: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut counts: HashMap<(usize, usize), bool> = HashMap::new();

    // 前処理
    for (x, y) in xy.iter() {
        match x_houses.get_mut(x) {
            Some(v) => {
                v.push(*y);
            }
            None => {
                x_houses.insert(*x, vec![*y]);
            }
        }
        match y_houses.get_mut(y) {
            Some(v) => {
                v.push(*x);
            }
            None => {
                y_houses.insert(*y, vec![*x]);
            }
        }
    }

    for (d, c) in dc.iter() {
        match d {
            'U' => {}
            'D' => {}
            'L' => {}
            'R' => {}
            _ => {
                panic!("unexpected cmd")
            }
        }
    }
}

fn mv_santa() {
    // サンタを移動させて座標をupdateする
    // 新しい家を通過したらcount up
    // 家の通過の判定が難しそう。
    //  ex: x, y -> x + c, y に動くとき。
    //      y = y, x < x <= x + c に家があるならその座標を取得する。
}

fn get_houses() {}