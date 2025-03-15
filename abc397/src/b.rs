use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut count = 0;

    if s[0] != 'i' {
        count += 1;
    }
    if s[s.len() - 1] != 'o' {
        count += 1;
    }

    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
