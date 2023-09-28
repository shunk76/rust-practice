// `Shape` traitを定義し、それに`perimeter`というメソッドを持たせる
// `Rectangle`と`Circle`という2つのstructを定義する
// 各structで`Shape` traitを実装し、`perimeter`メソッドをオーバーライドする
// `main`関数で`Rectangle`と`Circle`オブジェクトを生成し、それぞれの周囲の長さを表示する
use std::f64::consts::PI;

trait Shape {
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let rectangle = Rectangle::new(100.0, 200.0);
    let circle = Circle::new(15.0);

    println!("長方形の外周の長さは{:.1}", rectangle.perimeter());
    println!("円の外周の長さは{:.1}", circle.perimeter());
}
