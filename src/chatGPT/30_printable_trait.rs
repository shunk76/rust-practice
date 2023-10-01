trait Printable {
    fn print(&self);
}

struct Circle {
    radius: f64,
}

impl Printable for Circle {
    fn print(&self) {
        println!("This is a circle with radius: [{}]", self.radius)
    }
}

struct Square {
    side: f64,
}

impl Printable for Square {
    fn print(&self) {
        println!("This is a square with side: [{}]", self.side)
    }
}

// CircleにPrintableトレイトを実装してください。
// printメソッドが呼ばれたら、"This is a circle with radius: [radius]"と出力してください。

// SquareにPrintableトレイトを実装してください。
// printメソッドが呼ばれたら、"This is a square with side: [side]"と出力してください。

fn print_shapes<T: Printable>(shapes: Vec<T>) {
    for shape in shapes {
        shape.print();
    }
}

fn main() {
    // ここにコードを追加して、print_shapes関数を呼び出してください。
    // 出力は、各形状のprintメソッドによって定義されたメッセージになるはずです。
    print_shapes(vec![Circle { radius: 10.0 }, Circle { radius: 15.0 }]);
    print_shapes(vec![Square { side: 30.0 }, Square { side: 15.0 }]);
}
