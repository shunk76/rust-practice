// P.314

// ジェネリクスで関数定義
// fn 関数名<T: トレイト>(arg1: T, arg2: T) -> 戻り値の型 {}
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25)); // 型を明示するとき
    println!("{}", add('a', 'a')); // エラーになる
}
