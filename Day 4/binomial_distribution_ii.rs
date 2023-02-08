fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn get_bin_dist(x: u64, n: u64, p: f64, q: f64) -> f64 {
    (factorial(n) as f64) / ((factorial(x) as f64) * (factorial(n - x) as f64))
        * (p.powf(x as f64))
        * (q.powf((n - x) as f64))
}

fn main() {
    let p = 0.12;
    let q = 1.0 - p;
    let n = 10;
    let at_most = (0..3).map(|x| get_bin_dist(x, n, p, q)).sum::<f64>();
    let at_least = (2..=10).map(|x| get_bin_dist(x, n, p, q)).sum::<f64>();
    println!("{:.3}\n{:.3}", at_most, at_least);
}
