fn main() {
    let g1 = String::from("Hello");
    // ここで g1 の所有権が移動した
    show_message(g1);
    // ここでエラー
    println!("{}", g1);
}

fn show_message(message: String) {
    println!("{}", message);
}
