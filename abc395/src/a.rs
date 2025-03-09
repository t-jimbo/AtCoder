use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut flag = true;
    for i in 0..n {
        if i == 0 {
            continue;
        }

        if a[i] <= a[i - 1] {
            flag = false;
            break;
        }
    }

    println!("{}", if flag { "Yes" } else { "No" });
}
