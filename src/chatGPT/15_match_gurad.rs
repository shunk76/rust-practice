fn main() {
    let x = 16; // この値を変えてテストしてください。

    let result = match x {
        n if n == 0 => "zero".to_string(),
        _ => x.to_string(),
    };

    println!("{}", result);
}
