use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    for i in (1..s.len()).rev() {
        if &s[i - 1..i + 1] == "WA" {
            s.replace_range(i - 1..i + 1, "AC");
        }
    }

    println!("{}", s);
}