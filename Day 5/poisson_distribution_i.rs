fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let l: f64 = 2.5;
    let k: u64 = 5;
    let e: f64 = 2.71828;
    println!(
        "{:.3}",
        (l.powf(k as f64) * e.powf(-l)) / factorial(k) as f64
    );
}
