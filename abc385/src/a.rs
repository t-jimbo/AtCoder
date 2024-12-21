use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    std::println!("{}", if is_splittable(a, b, c) { "Yes" } else { "No" })
}

fn is_splittable(a: usize, b: usize, c: usize) -> bool {
    // 3つのグループに分けられるか
    if a == b && b == c {
        return true;
    }

    // 2つのグループに分けられるか
    a + b == c || b + c == a || c + a == b
}