// P.323

fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];

    // ここで所有権が移動する
    for a in array {
        println!("{}", a);
    }

    println!("len = {}", array.len()); // エラー
}
