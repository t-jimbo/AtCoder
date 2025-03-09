use num::integer::gcd;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut gcd_graph = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            gcd_graph[i][j] = gcd(a[i], a[j]);
            gcd_graph[j][i] = gcd_graph[i][j];
        }
    }

    for i in 0..n {
        let mut gcds = gcd_graph[i];
        // desc sort
        gcds.sort_by(|a, b| b.cmp(a));
        let mut max_gcd = 1;
        let mut taken = 0;
        for j in 0..k {
            max_gcd = gcd(max_gcd, gcds[j]);
        }
    }

    println!("{:?}", gcd_graph);
}