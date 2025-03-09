use proconio::{input, marker::Chars};

fn rev(c: char) -> char {
    match c {
        'N' => 'S',
        'S' => 'N',
        'E' => 'W',
        'W' => 'E',
        _ => c,
    }
}

fn main() {
    input! {
        d: Chars,
    }

    for c in d {
        print!("{}", rev(c));
    }

    println!();
}
