use proconio::input;
use std::collections::HashMap;

type Houses = HashMap<isize, Vec<isize>>;

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
    let mut count = 0;

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
                count += count_houses(&mut x_houses, &sx, sy + 1, sy + c);
                sy += c;
            }
            'D' => {
                count += count_houses(&mut x_houses, &sx, sy - c, sy - 1);
                sy -= c;
            }
            'L' => {
                count += count_houses(&mut y_houses, &sy, sx - c, sx - 1);
                sx -= c;
            }
            'R' => {
                count += count_houses(&mut y_houses, &sy, sx + 1, sy + c);
                sx += c;
            }
            _ => {
                panic!("unexpected cmd")
            }
        }
    }

    println!("{} {} {}", sx, sy, count);
}

fn count_houses(houses: &mut Houses, base: &isize, l: isize, r: isize) -> usize {
    let mut count: usize = 0;
    match houses.get_mut(base) {
        Some(v) => {
            let v_read = v.clone();
            for i in 0..v.len() {
                let h = v_read[i];
                if l <= h && h <= r {
                    // 移動する範囲内に家がある場合
                    println!("house: {}, {}", base, h);
                    count += 1;
                    v.remove(i); // 訪問済みの場合popする
                }
            }
        }
        None => {} // 家を通過しない
    };
    count
}
