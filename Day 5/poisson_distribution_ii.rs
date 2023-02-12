fn main() {
    let l1 = 0.88;
    let c1 = 160.0 + (40.0 * (l1 + l1 * l1));
    let l2 = 1.55;
    let c2 = 128.0 + (40.0 * (l2 + l2 * l2));
    println!("{:.3}\n{:.3}", c1, c2);
}
