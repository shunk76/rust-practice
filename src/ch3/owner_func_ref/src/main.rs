// P.218

fn main() {
    let g1 = String::from("Hello");
    show_message(&g1);
    println!("{}", g1);
}

// 参照を引数として渡す
fn show_message(message: &String) {
    println!("{}", message);
}
