use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_n: [usize; n], // 小学生の家の位置
        mut b_n: [usize; n], // 小学校の位置
    }

    // sort
    a_n.sort();
    b_n.sort();

    // 端からマッチングしていけばOK
    let mut res = 0;
    for i in 0..n {
        res += a_n[i].abs_diff(b_n[i]);
    }

    std::println!("{}", res);
}

