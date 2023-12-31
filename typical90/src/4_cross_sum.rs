fn main() {
    let h_w = &parse_input(1)[0];
    let a = &parse_input(h_w[0]);

    let row_sum = a.iter().map(|row| row.iter().sum()).collect::<Vec<usize>>();
    let col_sum = (0..h_w[1]).map(|j| a.iter().map(|row| row[j]).sum()).collect::<Vec<usize>>();
    for i in 0..h_w[0] {
        let sum_i = (0..h_w[1]).map(|j| (row_sum[i] + col_sum[j] - a[i][j]).to_string()).collect::<Vec<_>>();
        println!("{}", sum_i.join(" "));
    }
}


// stdin
fn parse_input(n: usize) -> Vec<Vec<usize>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let v = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v2.push(v);
    }
    v2
}
