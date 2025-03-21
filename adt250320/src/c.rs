use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;
    let mut current = 'A';
    for c in s {
        match c {
            'A' => {
                if current != 'A' {
                    ok = false;
                    break;
                }
            }
            'B' => {
                match current {
                    'A' => current = 'B',
                    'B' => {}
                    'C' => {
                        ok = false;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            'C' => {
                if current != 'C' {
                    current = 'C';
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
