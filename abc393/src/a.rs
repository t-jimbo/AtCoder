use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    if s1 == "sick" {
        if s2 == "sick" {
            println!("1");
        } else {
            println!("2");
        }
    } else {
        if s2 == "sick" {
            println!("3");
        } else {
            println!("4");
        }
    }
}
