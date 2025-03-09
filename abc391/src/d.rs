use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(usize, usize); n],
        q: usize,
        ta: [(usize, usize); q],
    }

    // (a, y)
    let mut columns: Vec<Vec<(usize, usize)>> = vec![vec![]; w];
    for i in 0..n {
        let (x, y) = xy[i];
        columns[x - 1].push((i, y - 1));
    }
    for column in columns.iter_mut() {
        column.sort_by(|a, b| a.1.cmp(&b.1));
    }

    let mut del_layer = usize::MAX; // max layer to be deleted
    let mut a_layers: Vec<usize> = vec![0; n]; // layer block A is in
    let mut layer_highest: Vec<usize> = vec![0; n]; // highest block y in each layer

    for i in 0..w {
        let cols = columns[i].len();
        // find the min layer
        del_layer = min(del_layer, cols);

        for j in 0..cols {
            let (a, y) = columns[i][j];
            a_layers[a] = j;
            layer_highest[j] = max(layer_highest[j], y);
        }
    }

    for i in 0..q {
        let (t, a) = ta[i];
        let layer = a_layers[a - 1];

        // never deleted
        if layer >= del_layer {
            println!("Yes");
            continue;
        }

        let del_time = layer_highest[layer] + 1;
        if t < del_time {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
