// P.125

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let mut counts = [0; 6];

    let end = 10000;
    let f_end = end as f64;

    for _ in 0..end {
        let dice = rng.gen_range(1..=6);
        counts[dice - 1] += 1;
    }

    println!("出目の回数: {:?}", counts);
    counts
        .iter()
        .enumerate()
        .for_each(|(i, &x)| println!("{}の確率: {}", i + 1, x as f64 / f_end));
}
