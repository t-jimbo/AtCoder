use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }

    s.sort_by(|a, b| a.len().cmp(&b.len()));

    println!("{}", s.join(""));
}