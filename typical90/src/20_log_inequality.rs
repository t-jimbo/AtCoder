use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }

    let a_is_smaller = a < c.pow(b);
    std::println!("{}", if a_is_smaller { "Yes" } else { "No" })
}
