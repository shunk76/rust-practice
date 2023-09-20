// P.279

use std::time::{SystemTime, UNIX_EPOCH};

// グローバル変数として乱数の種を指定
static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoch.as_millis() as u32;
    }

    // ^= 排他的論理和の代入演算子。 a ^= b は a = a ^ b と同義
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    for _ in 1..=10 {
        unsafe {
            let v = rand_global(1, 6);
            println!("{}", v)
        }
    }
}
