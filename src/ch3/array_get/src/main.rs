// P.234

fn main() {
    let month = ["Jan", "Feb", "Mar", "Apr", "May"];
    println!("{}", month[0]);

    // 最後の要素を取得
    let last = month.last();
    match last {
        Some(val) => println!("{}", val),
        None => println!("配列は空です。"),
    }
}
