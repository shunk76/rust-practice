// P.137

use rand::seq::SliceRandom;

const GRID_SIZE: usize = 5; // 奇数
const FREE_SPACE_INDEX: usize = (GRID_SIZE * GRID_SIZE - 1) / 2;

fn main() {
    let mut numbers: Vec<i32> = (1..=75).collect();

    numbers.shuffle(&mut rand::thread_rng());
    println!("{:?}", numbers);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let i = y * GRID_SIZE + x;

            if i == FREE_SPACE_INDEX {
                print!("  *,");
            } else {
                print!("{:3},", numbers[i]);
            }
        }
        println!("");
    }
}
