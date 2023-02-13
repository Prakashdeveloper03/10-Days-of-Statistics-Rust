use std::f64::consts::SQRT_2;

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

fn cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / sigma;
    0.5 * (1.0 + erf(z / (SQRT_2)))
}

fn main() {
    let x = 250.0;
    let n = 100.0;
    let mu = 2.4;
    let sigma = 2.0;
    let mu_sum = n * mu;
    let sigma_sum = ((n as f64).sqrt()) * sigma;
    println!("{:.4}", cdf(x, mu_sum, sigma_sum));
}
