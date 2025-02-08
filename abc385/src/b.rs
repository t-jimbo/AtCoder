use std::collections::HashMap;
use proconio::input;
use proconio::marker::Chars;

type CountMap = HashMap<(usize, usize), bool>;

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut x: usize,
        mut y: usize,
        s: [Chars; h],
        t: Chars,
    }

    // 家を通過した数。座標をkeyとする
    let mut counts: CountMap = HashMap::new();

    for cmd in t.iter() {
        match cmd {
            'U' => {
                // (x−1,y) が通行可能なら移動
                mv(s[x - 2][y - 1], (x - 1, y), &mut counts, || x = x - 1);
            }
            'D' => {
                mv(s[x][y - 1], (x + 1, y), &mut counts, || x = x + 1);
            }
            'L' => {
                mv(s[x - 1][y - 2], (x, y - 1), &mut counts, || y = y - 1);
            }
            'R' => {
                mv(s[x - 1][y], (x, y + 1), &mut counts, || y = y + 1);
            }
            _ => {
                // マスに留まる
            }
        };
    };

    println!("{} {} {}", x, y, counts.len())
}

fn mv<F>(c: char, p: (usize, usize), counts: &mut CountMap, move_fn: F)
where
    F: FnOnce(),
{
    match c {
        '#' => {
            // 通行不可なので何もしない
        }
        '.' => {
            // 通行できる
            move_fn();
        }
        '@' => {
            // 通行できる
            move_fn();
            // 通過(upsert)
            counts.insert(p, true);
        }
        cell => {
            panic!("unexpected cell char: {}", cell)
        }
    }
}