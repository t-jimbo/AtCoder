fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s
}

pub fn parse_usize() -> usize {
    let s = read_line();
    s.trim().parse::<usize>().unwrap()
}

pub fn parse_usize_vec() -> Vec<usize> {
    let s = read_line();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

pub fn parse_usize_vec_vertical(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(parse_usize());
    }

    v
}
