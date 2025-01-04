use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut count = 0;

    for i in 0..9 {
        for j in 0..9 {
            let sum = (i + 1) * (j + 1);
            if sum != x {
                count += sum;
            }
        }
    }

    println!("{}", count);
}
