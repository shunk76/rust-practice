fn main() {
    let price = 3950;
    // 500 * 10
    // 100 * 3
    // 50 * 10

    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = i500 * 500 + i100 * 100 + i50 * 50;
                if price == total {
                    println!(
                        "500円*{}枚 + 100円*{}枚 + 50円*{}枚 ({}枚)",
                        i500,
                        i100,
                        i50,
                        i500 + i100 + i50
                    );
                }
            }
        }
    }
}
