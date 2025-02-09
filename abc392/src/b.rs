use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    a.sort_by(|a, b| a.cmp(b));

    let mut k: usize = 0;
    let mut res: Vec<usize> = Vec::new();
    for i in 1..=n {
        if i < a[k] {
            res.push(i);
        } else if i == a[k] {
            if k + 1 == m {
                continue;
            }
            k += 1;
        } else {
            res.push(i);
        }
    }

    println!("{}", res.len());
    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
