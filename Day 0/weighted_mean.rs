use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n = input.trim().parse::<usize>().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let weights = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let weighted_mean = (numbers
        .iter()
        .zip(weights.iter())
        .map(|(a, b)| a * b)
        .sum::<i32>() as f64)
        / weights.iter().sum::<i32>() as f64;

    println!("{:.1}", weighted_mean);
}
