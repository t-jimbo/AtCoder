use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        if sum < 0 {
            sum = 0;
        }
    }

    println!("{}", sum);
}
