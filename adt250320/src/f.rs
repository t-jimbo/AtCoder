use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    // convert to bit
    let all = (1 << m) - 1;
    let bit_rows: Vec<usize> = s.iter().map(|row| row.iter().enumerate()
        .fold(0, |bit_row, (i, &c)| bit_row | ((c == 'o') as usize) << i))
        .collect();


    let mut min_rows = m;
    for mask in 1..(1 << n) {
        let mut bit_cols = 0;
        let mut count = 0;

        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                bit_cols |= bit_rows[i];
                count += 1;
            }
        }

        if bit_cols == all {
            min_rows = min_rows.min(count);
        }
    }

    println!("{}", min_rows);
}
