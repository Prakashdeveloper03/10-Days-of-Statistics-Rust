fn probability(x: f64, mu: f64, std: f64) -> f64 {
    let z = (x - mu) / (std * std::f64::consts::SQRT_2);
    (1.0 + erf(z)) / 2.0 * 100.0
}

fn erf(z: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    let sign = if z < 0.0 { -1.0 } else { 1.0 };
    let z = z.abs();
    let t = 1.0 / (1.0 + p * z);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * f64::exp(-z * z);
    return sign * y;
}

fn main() {
    let mu = 70.0;
    let std = 10.0;
    let s1 = 80.0;
    let s2 = 60.0;
    let higher = 100.0 - probability(s1, mu, std);
    let passed = 100.0 - probability(s2, mu, std);
    let failed = probability(s2, mu, std);
    println!("{:.2}", higher);
    println!("{:.2}", passed);
    println!("{:.2}", failed);
}
