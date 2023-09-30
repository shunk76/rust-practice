use std::f64::consts::PI;

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Area for Circle {
    // 円の面積を返すメソッドをここに実装してください。
    fn area(&self) -> f64 {
        self.radius.powf(2.0) * PI
    }
}

impl Area for Square {
    // 正方形の面積を返すメソッドをここに実装してください。
    fn area(&self) -> f64 {
        self.side.powf(2.0)
    }
}

fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    // 以下のコードを完成させて、それぞれの形状の面積を出力してください。
    // Hint: print_area関数を使用してください。
    let circle = Circle { radius: 10.0 };
    let square = Square { side: 10.0 };

    print_area(circle);
    print_area(square);
}
