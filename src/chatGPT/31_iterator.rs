trait SimpleIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

// ここに SimpleIterator トレイトを Counter に対して実装してください。
// next メソッドは、count が max に到達するまで count の値を返し、
// max に到達した場合は None を返すようにしてください。

impl SimpleIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.max {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}

fn main() {
    let mut counter = Counter::new(5);

    // 以下のコードで、Counter の各値を出力できるようにしてください。
    // Hint: next メソッドを使用してください。

    for _ in 0..6 {
        println!("{:?}", counter.next());
    }
}
