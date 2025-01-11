use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n], // sorted
    }

    let mut count = 0;

    // 上のモチ
    for i in 0..n {
        if a[i] * 2 > a[n - 1] {
            // 一番大きいやつの半分より大きかったらもうない
            break;
        }

        // 下のモチ
        for j in (i + 1)..n {
            if a[i] * 2 <= a[j] {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
