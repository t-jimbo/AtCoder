use core::ffi::c_ptrdiff_t;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let l_digits: Vec<usize> = l.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let r_digits: Vec<usize> = r.to_string().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let l_digit_len = l_digits.len(); // 最大見る桁数
    let r_digit_len = r_digits.len(); // 最大見る桁数
    let l_first = l_digits[0];
    let r_first = r_digits[0];

    let mut count = 0;

    // lの桁~rの桁までを走査する
    for d in l_digit_len..=r_digit_len {
        // 先頭の数
        for i in 1..=9 {
            // d桁で、iから始まる数で、lからrの間のsnake_numが何個あるか。

            // lと同じ桁の場合
            if d == l_digit_len {
                if i < l_first {
                    // 先頭がlの先頭より小さいなら見なくてOK
                    continue;
                }
                if i == l_first {
                    // 
                    let mut r_count = 1;
                    for dr in 1..r_digit_len {
                        let m = cmp::min(r_first, r_digits[dr]);
                        r_count *= m;
                    }
                    count += r_count;
                    continue;
                }
            }

            // rと同じ桁の場合
            if d == r_digit_len {
                if i > r_first {
                    // 先頭の位が大きい場合は終了
                    break;
                }
                if i == r_first {
                    // rの各桁と、iよりも小さい数を数える。
                    let mut r_count = 1;
                    for dr in 1..r_digit_len {
                        let m = cmp::min(r_first, r_digits[dr]);
                        r_count *= m;
                    }
                    count += r_count;
                    continue;
                }
            }

            // countする
            count += (i.pow(d));
        }
    }

    println!("{}", count);
}
