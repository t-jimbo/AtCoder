use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;

    for i in 1..s.len() {
        if i == 1 {
            if s[0] == 'o' && s[1] == 'o' {
                ok = false;
                break;
            }
            continue;
        }

        if s[i] == 'o' {
            if s[i - 1] == 'o' || s[i - 2] == 'o' {
                ok = false;
                break;
            }
        } else {
            if s[i - 1] == 'x' && s[i - 2] == 'x' {
                ok = false;
                break;
            }
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
