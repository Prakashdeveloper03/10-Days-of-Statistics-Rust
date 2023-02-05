use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let mean = numbers.iter().sum::<i32>() as f64 / n as f64;

    let median = if n % 2 != 0 {
        numbers[n / 2] as f64
    } else {
        (numbers[n / 2] as f64 + numbers[(n / 2) - 1] as f64) / 2.0
    };

    let mut occurrences = std::collections::HashMap::new();
    for &n in numbers.iter() {
        let count = occurrences.entry(n).or_insert(0);
        *count += 1;
    }

    let mode = *occurrences
        .iter()
        .filter(|(_, &count)| count == count)
        .map(|(value, _)| value)
        .min()
        .unwrap();

    println!("{:.1}\n{:.1}\n{}", mean, median, mode);
}
