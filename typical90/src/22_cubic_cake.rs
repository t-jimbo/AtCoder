use proconio::input;

fn main() {
    input! {
        w: usize,
        d: usize,
        h: usize,
    }

    std::println!("{}", cut(w, d, h));
}

fn cut(w: usize, d: usize, h: usize) -> usize {
    if w == d && d == h {
        return 0;
    }

    if w == d && h % d == 0 {
        return h / d - 1;
    }
    if d == h && w % h == 0 {
        return w / h - 1;
    }
    if h == w && d % w == 0 {
        return d / w - 1;
    }

    // 全カット
    w + d + h - 3
}