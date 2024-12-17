fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<usize>().unwrap();

    if n % 2 != 0 {
        return;
    }

    solve(n);
}

fn is_valid(s: &str) -> bool {
    let mut count = 0;
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count < 0 {
            return false;
        }
    }
    count == 0
}

// bit全探索する
fn solve(n: usize) {
    for i in 0..1 << n {
        let mut s = String::new();
        for j in (0..n).rev() {
            if i & 1 << j == 0 {
                s += "("
            } else {
                s += ")"
            }
        }
        if is_valid(&s) {
            println!("{}", s);
        }
    }
}
