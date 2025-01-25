use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }

    let mut r: f64 = 0.0;
    let mut ok = true;

    for i in 1..n {
        if i == 1 {
            r = a[i] / a[i - 1];
            continue;
        }

        if a[i] != r * a[i - 1] {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
