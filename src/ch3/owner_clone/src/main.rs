// P.213

fn main() {
    let g1 = String::from("Hello");
    // 複製すれば所有権は移動しない
    let g2 = g1.clone();
    println!("{}", g1);
    println!("{}", g2);
}
