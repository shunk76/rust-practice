// P.317
// 構造体でジェネリクスを指定する

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let pt_i = Point { x: 20, y: 50 };
    let pt_f = Point { x: 20.5, y: 15.3 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);
}
