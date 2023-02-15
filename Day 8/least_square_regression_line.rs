fn predict(w0: f64, w1: f64, x: i32) -> f64 {
    return w0 + (w1 * x as f64);
}

fn main() {
    let x: Vec<i32> = vec![95, 85, 80, 70, 60];
    let y: Vec<i32> = vec![85, 95, 70, 65, 70];
    let x_mean = x.iter().sum::<i32>() as f64 / x.len() as f64;
    let y_mean = y.iter().sum::<i32>() as f64 / y.len() as f64;
    let mut cov = 0.0;
    let mut var = 0.0;
    for (i, value) in x.iter().enumerate() {
        cov += (y[i] as f64 - y_mean) * (*value as f64 - x_mean);
        var += (*value as f64 - x_mean).powi(2);
    }
    let w1 = cov / var;
    let w0 = y_mean - (w1 * x_mean);
    println!("{:.3}", predict(w0, w1, 80));
}
