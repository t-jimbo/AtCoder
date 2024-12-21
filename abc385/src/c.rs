use std::collections::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }


    // 1. 高さをkeyとして、ビルのindexの配列をつくる
    let mut builgings: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in n {
        builgings.insert(h[i], i);
    }

    let mut max = 0;
    // 2. それぞれに対して等間隔で最大いくつになるかチェックする
    for (_, indexes) in builgings.iter() {
        // 1, 3, 5, 8
    }
}