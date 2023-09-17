// P.233

fn main() {
    // 配列
    let points = [80, 40, 50, 90, 84];
    // 型を省略しない場合
    // let points: [i32; 5] = [80, 40, 50, 90, 84];
    println!("{:?}", points);
    // 要素数を表示
    println!("length = {}", points.len());
}
