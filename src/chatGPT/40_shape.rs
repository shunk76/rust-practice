// Shape という trait を作成してください。この trait は area というメソッドを持つようにしてください。
// Rectangle と Circle の2つの struct を作成し、これらに Shape トレイトを実装してください。
// area メソッドは、それぞれの形状の面積を計算して返すようにしてください（円の場合はπは3.14159としてよい）。
// main 関数で、Rectangle と Circle の area メソッドを呼び出し、結果を表示してください。

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powf(2.0) * 3.14159
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 200.0,
        height: 100.5,
    };

    let circle = Circle { radius: 11.0 };

    println!("長方形の面積は {}", rectangle.area());
    println!("円の面積は {}", circle.area());
}
