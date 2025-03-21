use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q],
    }

    for i in 0..q {
        let idx = l[i] - 1;
        let prev = a[idx];
        if prev == n {
            continue;
        }

        if idx == k - 1 || a[idx + 1] > prev + 1 {
            a[idx] += 1;
        }
    }


    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
