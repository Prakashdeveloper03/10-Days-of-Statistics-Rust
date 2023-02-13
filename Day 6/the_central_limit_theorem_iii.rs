fn main() {
    let n = 100.0;
    let mu = 500.0;
    let sigma = 80.0;
    let z = 1.96;
    let sample_std = sigma / ((n as f64).sqrt());
    let a = mu - (z * sample_std);
    let b = mu + (z * sample_std);
    println!("{:.2}\n{:.2}", a, b);
}
