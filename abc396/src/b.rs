use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = Vec::new();
    for _ in 0..q {
        input! {
            c: usize,
        }

        match c {
            1 => {
                input! {
                    x: usize,
                }
                stack.push(x);
            }
            2 => {
                if let Some(x) = stack.pop() {
                    println!("{}", x);
                } else {
                    println!("0");
                }
            }
            _ => {
                panic!("Invalid input");
            }
        }
    }
}
