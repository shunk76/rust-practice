// add_values というジェネリクスを用いた関数を作成してください。
// この関数は、任意の型 T の2つの Option<T> 値を受け取ります。
// 両方の Option<T> が Some であれば、その2つの値を加算（+ 演算子を使用）して Some で返します。
// どちらかが None であれば、None を返します。
// ただし、T は Add トレイトを実装している必要があります。

fn add_values<T: std::ops::Add<Output = T>>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    }
}

fn main() {
    println!("{:?}", add_values(Some(10), Some(10)));
    println!("{:?}", add_values(Some(10.0), Some(10.5)));
    println!("{:?}", add_values(Some(10), None));
    println!("{:?}", add_values::<i32>(None, None));
}
