use proconio::input;

fn main() {
    input! {
        n: u32,
        ab: [(u32, u32); n - 1],
    }
    println!("{:?}", ab);
}
