// Shapeという名前のトレイトを作成し、areaというメソッドを持つようにしてください。
// このメソッドは図形の面積をf64型で返すものとします。
// RectangleとCircleの2つの構造体を定義してください。
// Rectangleはwidthとheightというフィールドを持ちます。
// Circleはradiusというフィールドを持ちます。
// それぞれの構造体に対して、Shapeトレイトを実装してください。
// 最後に、main関数でそれぞれの図形の面積を計算し、出力してください。

use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powf(2.0) * PI
    }
}

fn main() {
    let rectangle = Rectangle::new(100.0, 100.0);
    println!("長方形の面積は{:.1}", rectangle.area());

    let circle = Circle::new(10.0);
    println!("円の面積は{:.1}", circle.area())
}
