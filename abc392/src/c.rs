use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut pq: Vec<(usize, usize)> = p.iter().zip(q.iter()).map(|(&pi, &qi)| (pi, qi)).collect();
    pq.sort_by_key(|&(_, qi)| qi);

    let mut res: Vec<usize> = Vec::new();
    for (pi, _) in pq {
        res.push(q[pi - 1]);
    }

    println!("{}", res.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}
