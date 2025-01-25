use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    // 左上の頂点
    let mut top_left: Option<(usize, usize)> = None;
    let mut min_right_available: Option<(usize, usize)> = None;
    let mut min_left_available: Option<(usize, usize)> = None;

    let max_left_black = 0;

    let mut max_right_black = 0;
    let mut bottom = 0;

    let mut ok = true;

    for i in 0..h {

        // その行のmin right available
        let mut row_min_right_available: Option<(usize, usize)> = None;
        let mut row_min_left_available: Option<(usize, usize)> = None;

        for j in 0..w {
            // update right, bottom
            if s[i][j] == '#' {
                // always update
                bottom = i;

                // update max right
                if max_right_black < j {
                    max_right_black = j;
                }
                if max_right_black > j {
                    max_right_black = j;
                }
            }


            match top_left {
                Some(top) => {
                    match row_min_right_available {
                        Some(_) => {
                            // 飛び地になってたらout
                            if s[i][j] == '#' {
                                ok = false;
                                // println!("split black");
                                break;
                            }
                        }
                        None => {
                            if top.1 <= j && s[i][j] == '.' {
                                // いけなくなったら更新
                                row_min_right_available = Some((i, j));
                            }
                        }
                    }

                    // bottomを更新しつつ、max_left_blackより右かつ、max_right_blackより左で白があったらout
                    if bottom == i && max_left_black <= j && j <= max_right_black && s[i][j] == '.' {
                        ok = false;
                        // println!("white is appeared");
                        break;
                    }
                }

                None => {
                    if s[i][j] == '#' {
                        top_left = Some((i, j));
                    }
                }
            }
        }

        // rowごとのcheck
        match min_right_available {
            Some(min) => {
                // bottomを更新しつつ、これまでのminを、max_blackが超えてしまったらout
                if bottom == i && min.1 < max_right_black {
                    ok = false;
                    // println!("over max black");
                    break;
                }
            }
            None => {}
        }
        // minを更新
        min_right_available = row_min_right_available;
    }

    // println!("console: {:?}", min_right_available);

    println!("{}", if ok { "Yes" } else { "No" });
}
