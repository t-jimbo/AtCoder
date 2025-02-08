use proconio::input;

fn main() {
    input! {
        a: [usize; 5]
    }

    let mut op_count = 0;
    for i in 0..5 {
        if a[i] == i + 1 {
            continue;
        }

        if i != 4 && a[i] == i + 2 {
            op_count += 1;
            continue;
        }

        break;
    }

    println!("{}", if op_count == 1 { "Yes" } else { "No" });
}
