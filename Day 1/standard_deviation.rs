use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums = nums
        .trim()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mean = nums.iter().sum::<i32>() as f64 / n as f64;
    let squared_distances = nums
        .iter()
        .map(|num| (num - mean as i32).pow(2) as f64)
        .sum::<f64>();
    let standard_deviation = (squared_distances / n as f64).sqrt();

    println!("{:.1}", standard_deviation);
}
