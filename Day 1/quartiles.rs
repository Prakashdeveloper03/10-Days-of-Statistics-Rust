use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    numbers.sort();

    let q2 = if n % 2 != 0 {
        numbers[n / 2]
    } else {
        (numbers[n / 2] + numbers[n / 2 - 1]) / 2
    };

    let lower = &numbers[..n / 2];
    let upper = if n % 2 != 0 {
        &numbers[n / 2 + 1..]
    } else {
        &numbers[n / 2..]
    };

    let n_lower = lower.len();
    let n_upper = upper.len();

    let q1 = if n_lower % 2 != 0 {
        lower[n_lower / 2]
    } else {
        (lower[n_lower / 2] + lower[n_lower / 2 - 1]) / 2
    };

    let q3 = if n_upper % 2 != 0 {
        upper[n_upper / 2]
    } else {
        (upper[n_upper / 2] + upper[n_upper / 2 - 1]) / 2
    };

    println!("{}\n{}\n{}", q1, q2 as i32, q3);
}
