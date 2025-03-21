use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let s: String = (0..k)
        .map(|i| (b'A' + i as u8) as char)
        .collect();

    println!("{}", s);
}
