use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error reading input");
    let _n: usize = input.trim().parse().expect("error parsing input");

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

    let out = spearman(&x, &y);
    println!("{:.3}", out);
}

fn rank(arr: &[f64]) -> Vec<f64> {
    let mut arr_with_index: Vec<(usize, &f64)> = arr.iter().enumerate().collect();
    arr_with_index.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let mut ranks: Vec<f64> = vec![0.0; arr.len()];
    let mut i = 1;
    while i <= arr.len() {
        let mut j = i;
        while j < arr.len() && arr_with_index[j].1 == arr_with_index[i - 1].1 {
            j += 1;
        }
        let rank_sum = ((i + j) * (j - i + 1)) as f64 / 2.0;
        for k in i - 1..j {
            ranks[arr_with_index[k].0] = rank_sum;
        }
        i = j + 1;
    }
    return ranks;
}

fn spearman(x: &[f64], y: &[f64]) -> f64 {
    let rx = rank(&x);
    let ry = rank(&y);
    let n = x.len() as f64;
    let mut d: f64 = 0.0;
    for i in 0..x.len() {
        d += (rx[i] - ry[i]).powi(2);
    }
    let r = 1.0 - 6.0 * d / (n * (n.powi(2) - 1.0));
    return r;
}
