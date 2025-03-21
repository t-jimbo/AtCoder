use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;

    for i in 1..s.len() {
        if i == 1 && s[0] == 'o' && s[1] == 'o' {
            ok = false;
            break;
        }

        if s[i] == 'o' && s[i - 1] == 'o' || s[i - 2] == 'o' {
            ok = false;
            break;
        }
        if [i] == 'x' && s[i - 1] == 'x' && s[i - 2] == 'x' {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
