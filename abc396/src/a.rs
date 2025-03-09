use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;
    for i in 0..n {
        if i == 0 {
            count += 1;
            continue;
        }

        if a[i] == a[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        if count >= 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
