use std::f64::consts::PI;
use proconio::input;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        es: [f64; q],
    }

    for e_i in 0..q {
        let e = es[e_i]; // 時刻(min)
        let phi = calc_phi(e, t, l, x, y);
        std::println!("{}", phi)
    }
}

// 時刻eにおける、俯角Φを計算する
fn calc_phi(e: f64, t: f64, l: f64, x: f64, y: f64) -> f64 {
    // 観覧車の角Θ
    let theta = 2.0 * PI * e / t;
    let sin_theta = f64::sin(theta);
    let cos_theta = f64::cos(theta);

    // 観覧車の座標(0, u, v)
    let u = -l * 0.5 * sin_theta;
    let v = l * 0.5 * (1.0 - cos_theta);

    // xy平面での観覧車と像の距離
    let r = f64::sqrt(x.powi(2) + (y - u).powi(2));

    let tan = v / r;
    f64::atan(tan).to_degrees()
}