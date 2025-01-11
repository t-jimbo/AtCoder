use proconio::input;

fn main() {
    input! {
        n: usize,
        mut stones: [usize; n],
    }

    for i in 0..n {
        // i+1年後の世界。i+1人目(index: i)が成人。i人(0~i-1)がすでに成人している。
        if i == 0 {
            // 1人目が成人する年なので、石の移動は起きない
            continue;
        }

        let mut given = 0;
        for j in 0..i {
            if stones[j] > 0 {
                stones[j] -= 1;
                given += 1;
            }
        }
        stones[i] += given;
    }

    println!("{}", stones.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}
