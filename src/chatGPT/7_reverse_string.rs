// 文字列を逆順にする関数を実装してください。
// 例: "Hello, world!" -> "!dlrow ,olleH"

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let original = "Hello, world!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
