use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for c in s {
        if c == '2' {
            print!("2");
        }
    }
    println!();
}
