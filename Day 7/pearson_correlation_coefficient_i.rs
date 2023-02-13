use std::io;

fn get_std(l: &[f64], mu: f64, n: usize) -> f64 {
    (l.iter().map(|_l| (_l - mu).powi(2)).sum::<f64>() / n as f64).sqrt()
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let n: usize = input.trim().parse().expect("error parsing input");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let x: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("error parsing input"))
        .collect();

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let y: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("error parsing input"))
        .collect();

    let mu_x = x.iter().sum::<f64>() / n as f64;
    let std_x = get_std(&x, mu_x, n);

    let mu_y = y.iter().sum::<f64>() / n as f64;
    let std_y = get_std(&y, mu_y, n);

    let coef = x
        .iter()
        .zip(y.iter())
        .map(|(_x, _y)| (_x - mu_x) * (_y - mu_y))
        .sum::<f64>()
        / (n as f64 * std_x * std_y);

    println!("{:.3}", coef);
}
