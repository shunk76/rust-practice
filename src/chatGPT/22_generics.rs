// ジェネリックなPair<T>構造体を定義し、2つのジェネリックなフィールドxとyがあります。
// new(x: T, y: T) -> Pair<T> という名前の関連関数（static method）を実装し、
// 新しいPairを返すようにしてください。
// sum(&self) -> T というメソッドを返すようにしてください。
// ただし、このメソッドはTが数値である場合にのみ動作するように制約を付けてください。
// display(&self)というメソッドを実装して、内容をプリントできるようにしてください。
// このメソッドは、TがDisplayトレイトを実装している場合にのみ動作するように制約を付けてください。

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display + std::ops::Add<Output = T> + Copy> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }

    fn sum(&self) -> T {
        self.x + self.y
    }

    fn display(&self) {
        println!("x: {}, y: {}, 合計: {}", self.x, self.y, self.sum())
    }
}

fn main() {
    let pair1 = Pair::new(1, 3);
    let pair2 = Pair::new(10.0, 5.1);

    pair1.display();
    pair2.display();
}
