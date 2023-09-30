// ジェネリック型とtraitを使って、キュー(Queue)データ構造を実装してみましょう。
// 以下の条件を満たすQueue structとそのメソッドを実装してください。
// Queueはジェネリック型Tを持つ
// enqueue(&mut self, item: T)メソッド：アイテムをキューの末尾に追加
// dequeue(&mut self) -> Option<T>メソッド：キューの先頭のアイテムを取り出し、そのアイテムを返す。キューが空の場合はNoneを返す
// is_empty(&self) -> boolメソッド：キューが空ならtrue、それ以外ならfalseを返す
// size(&self) -> usizeメソッド：キューに格納されているアイテム数を返す

use std::collections::VecDeque;

trait Q<T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }
}

impl<T> Q<T> for Queue<T> {
    fn enqueue(&mut self, item: T) {
        self.items.push_back(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue("a");
    queue.enqueue("b");
    queue.enqueue("c");

    println!("{:?}", queue);

    for _ in 0..(queue.items.len() + 1) {
        println!("{:?}", queue.dequeue())
    }
}

// 問題文に忠実なコード

// trait Q<T> {
//     fn enqueue(&mut self, item: T);
//     fn dequeue(&mut self) -> Option<T>;
//     fn is_empty(&self) -> bool;
//     fn size(&self) -> usize;
// }

// #[derive(Debug)]
// struct Queue<T> {
//     items: Vec<T>,
// }

// impl<T> Queue<T> {
//     fn new() -> Self {
//         Queue { items: Vec::new() }
//     }
// }

// impl<T> Q<T> for Queue<T> {
//     fn enqueue(&mut self, item: T) {
//         self.items.push(item)
//     }

//     fn dequeue(&mut self) -> Option<T> {
//         if self.is_empty() {
//             None
//         } else {
//             Some(self.items.remove(0))
//         }
//     }

//     fn is_empty(&self) -> bool {
//         self.size() == 0
//     }

//     fn size(&self) -> usize {
//         self.items.len()
//     }
// }

// fn main() {
//     let mut queue = Queue::new();
//     queue.enqueue("a");
//     queue.enqueue("b");
//     queue.enqueue("c");

//     println!("{:?}", queue);

//     while !queue.is_empty() {
//         println!("{:?}", queue.dequeue());
//     }
//     println!("{:?}", queue.dequeue());
// }
