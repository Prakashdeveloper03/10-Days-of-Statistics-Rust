fn probability(x: f64, mu: f64, std: f64) -> f64 {
    let erf_value = erf((x - mu) / (std * f64::sqrt(2.0)));
    return (1.0 + erf_value) / 2.0;
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
    println!("{:.3}", probability(19.5, 20.0, 2.0));
    println!(
        "{:.3}",
        probability(22.0, 20.0, 2.0) - probability(20.0, 20.0, 2.0)
    );
}
