trait Add<T> {
    fn add(self, other: T) -> T;
}

// i32型に対してAdd<i32>トレイトを実装している
impl Add<i32> for i32 {
    fn add(self, other: i32) -> i32 {
        self + other
    }
}

impl Add<f64> for f64 {
    fn add(self, other: f64) -> f64 {
        self + other
    }
}

// Add traitを実装した型Tに対して動作するジェネリックな関数`generic_add`を作成してください。
// この関数は2つのT型の値を受け取り、それらを足し合わせた結果を返します。

// こちらがその関数のシグネチャです。
// fn generic_add<T: ???>(a: T, b: T) -> T {
//     // ここにコードを書いてください。
// }

fn generic_add<T: Add<T>>(a: T, b: T) -> T {
    a.add(b)
}

fn main() {
    // generic_add関数を呼び出して、結果を出力してください。
    // 例: println!("Addition: {}", generic_add(5, 10));
    println!("Addition: {}", generic_add(5, 10));
}
