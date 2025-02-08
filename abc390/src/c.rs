use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut appeared = false;
    let mut top = 0;
    let mut left = 0;
    let mut right = 0;
    let mut bottom = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if !appeared {
                    appeared = true;
                    top = i;
                    left = j;
                }

                bottom = i;
                if right < j {
                    right = j;
                }

                if left > j {
                    left = j;
                }
            }
        }
    }

    let mut ok = true;
    for i in top..=bottom {
        for j in left..=right {
            if s[i][j] == '.' {
                ok = false;
                break;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
