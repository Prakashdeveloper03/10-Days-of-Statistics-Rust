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
    let b = 1.09;
    let p = b / (b + 1.0);
    let q = 1.0 - p;
    let n = 6;
    let proportion = (3..7).map(|x| get_bin_dist(x, n, p, q)).sum::<f64>();
    println!("{:.3}", proportion);
}
