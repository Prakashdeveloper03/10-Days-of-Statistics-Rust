fn main() {
    let n = 1;
    let d = 3;
    let insp = 5;
    let p = n as f64 / d as f64;
    let q = 1.0 - p;
    println!(
        "{:.3}",
        (1..=insp)
            .map(|i| q.powf(((i - 1) as f32).into()) * p)
            .sum::<f64>()
    );
}
