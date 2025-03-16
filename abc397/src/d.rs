use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // k^3 <= n
    let n_cbrt = (n as f64).powf(1.0 / 3.0) as usize;
    let mut res: (usize, usize) = (0, 0);
    for k in 1..=n_cbrt {
        // x - y = k && xy = (n/k - k^2) / 3
        let nk3 = n - k.pow(3);
        if nk3 % (3 * k) != 0 {
            // x, y is not integer
            continue;
        }

        let xy = nk3 / (3 * k);
        let y_sqrt = ((k * k + 4 * xy) as f64).sqrt();
        if y_sqrt.fract() != 0.0 {
            // y is irrational
            continue;
        }
        let y_num = y_sqrt as usize - k;
        if y_num % 2 != 0 {
            // y is not integer
            continue;
        }

        let y = y_num / 2;
        if y == 0 {
            // y should be positive
            continue;
        }
        res = (y + k, y);
        break;
    }

    if res == (0, 0) {
        println!("-1");
    } else {
        println!("{} {}", res.0, res.1);
    }
}
