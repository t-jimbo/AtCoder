use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let m = n / 2;
    for i in 0..n {
        for j in 0..n {
            if i < m {
                // top half
                if j < m {
                    // left half
                    if j < i {
                        // outer
                        if j % 2 == 0 {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    } else {
                        // inner
                        if i % 2 == 0 {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    }
                } else {
                    // right half
                    if n - j <= i + 1 {
                        if (n - j) % 2 == 0 {
                            print!(".")
                        } else {
                            print!("#")
                        }
                    } else {
                        if i % 2 == 0 {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    }
                }
            } else {
                // bottom half
                if j < m {
                    // left half
                    if j < n - i {
                        // outer
                        if j % 2 == 0 {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    } else {
                        // inner
                        if (n - i) % 2 == 0 {
                            print!(".")
                        } else {
                            print!("#")
                        }
                    }
                } else {
                    // right half
                    if i <= j {
                        // outer
                        if (n - j) % 2 == 0 {
                            print!(".")
                        } else {
                            print!("#")
                        }
                    } else {
                        if (n - i) % 2 == 0 {
                            print!(".")
                        } else {
                            print!("#")
                        }
                    }
                }
            }
        }
        println!();
    }
}
