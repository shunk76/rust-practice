fn main() {
    {
        // s1 のスコープは{}内のみ
        let s1 = String::from("hello");
        println!("{}", s1);
    }

    println!("{}", s1); // error: s1 はスコープ外
}
