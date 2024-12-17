use proconio::input;

fn main() {
    input! {
        n: usize, // 生徒数
        cp: [[usize; 2]; n], // クラスと得点
        q: usize, // question数
        lr: [[usize; 2]; q], // 学籍番号の両端
    }

    std::println!("test");
    std::println!("{}", n);

    for i in 0..n {
        std::println!("{}, {}", cp[i][0], cp[i][1]);
    }

    for i in 0..q {
        std::println!("{}, {}", lr[i][0], lr[i][1]);
    }
}
