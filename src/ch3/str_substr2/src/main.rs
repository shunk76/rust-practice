// P.263

fn main() {
    let pr = "知恵は武器よりも価値がある。";

    let sub1: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub1);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars = &pr_chars[3..=4];
    let sub2: String = sub_chars.into_iter().collect();
    println!("4-5文字目: {}", sub2)
}
