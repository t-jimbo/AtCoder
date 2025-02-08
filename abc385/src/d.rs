use proconio::input;
use std::collections::HashMap;

type Houses = HashMap<isize, Vec<isize>>;
type VisitedMap = HashMap<(isize, isize), bool>;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut sx: isize, // サンタの座標
        mut sy: isize,
        xy: [(isize, isize); n], // 家
        dc: [(char, isize); m], // cmd
    }

    let mut x_houses: Houses = HashMap::new();
    let mut y_houses: Houses = HashMap::new();
    let mut visited: VisitedMap = HashMap::new();

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
            'U' => {
                count_houses(&x_houses, &mut visited, true, &sx, sy + 1, sy + c);
                sy += c;
            }
            'D' => {
                count_houses(&x_houses, &mut visited, true, &sx, sy - c, sy - 1);
                sy -= c;
            }
            'L' => {
                count_houses(&y_houses, &mut visited, false, &sy, sx - c, sx - 1);
                sx -= c;
            }
            'R' => {
                count_houses(&y_houses, &mut visited, false, &sy, sx + 1, sy + c);
                sx += c;
            }
            _ => {
                panic!("unexpected cmd")
            }
        }
    }

    println!("{} {} {}", sx, sy, visited.len());
}

fn count_houses(houses: &Houses, visited: &mut VisitedMap, is_x: bool, base: &isize, l: isize, r: isize) {
    match houses.get(base) {
        Some(v) => {
            for point in v {
                if l <= *point && *point <= r {
                    // 移動する範囲内に家がある場合
                    let h_point = if is_x { (*base, *point) } else { (*point, *base) };
                    if visited.contains_key(&h_point) {
                        // 訪問済みはskip
                        continue;
                    }

                    visited.insert(h_point, true);
                }
            }
        }
        None => {} // 家を通過しない
    };
}
