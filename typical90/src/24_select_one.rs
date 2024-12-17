use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_n: [usize; n],
        b_n: [usize; n],
    }

    // 必要なoperation count
    let mut oc = 0;
    for i in 0..n {
        oc += a_n[i].abs_diff(b_n[i]);
    }

    let convertible = k >= oc && (k - oc) % 2 == 0;

    std::println!("{}", if convertible { "Yes" } else { "No" })
}
