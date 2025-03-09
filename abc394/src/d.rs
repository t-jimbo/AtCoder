use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack: Vec<char> = Vec::new();
    for c in s {
        match c {
            '(' => {
                stack.push(c);
            }
            '[' => {
                stack.push(c);
            }
            '<' => {
                stack.push(c);
            }
            p => {
                if stack.is_empty() {
                    println!("No");
                    return;
                }
                let q = stack.pop().unwrap();
                match (q, p) {
                    ('(', ')') => {}
                    ('[', ']') => {}
                    ('<', '>') => {}
                    _ => {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    if !stack.is_empty() {
        println!("No");
        return;
    }
    println!("Yes");
}
