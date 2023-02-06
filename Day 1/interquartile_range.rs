use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let frequencies: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers = Vec::new();
    for (e, f) in elements.iter().zip(frequencies.iter()) {
        for _ in 0..*f {
            numbers.push(*e);
        }
    }
    numbers.sort();

    let n = numbers.len();
    let lower = &numbers[..n / 2];
    let upper = if n % 2 == 0 {
        &numbers[n / 2..]
    } else {
        &numbers[n / 2 + 1..]
    };

    let n_lower = lower.len();
    let q1 = if n_lower % 2 != 0 {
        lower[n_lower / 2]
    } else {
        (lower[n_lower / 2] + lower[n_lower / 2 - 1]) / 2
    };

    let n_upper = upper.len();
    let q3 = if n_upper % 2 != 0 {
        upper[n_upper / 2]
    } else {
        (upper[n_upper / 2] + upper[n_upper / 2 - 1]) / 2
    };

    println!("{:.1}", (q3 - q1) as f64);
}
