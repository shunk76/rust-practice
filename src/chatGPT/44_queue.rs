// 以下のようなジェネリックなQueue<T>という名前の構造体を作成してください。
// 内部でVec<T>を使用して要素を保持します。
// enqueue(&mut self, item: T) というメソッドを持ち、要素をキューの末尾に追加します。
// dequeue(&mut self) -> Option<T> というメソッドを持ち、キューの先頭の要素を削除し、
// その要素を返します。キューが空の場合はNoneを返します。
// peek(&self) -> Option<&T> というメソッドを持ち、キューの先頭の要素を参照として返します。
// キューが空の場合はNoneを返します。
// このQueue構造体に対する単体テストもいくつか書いてみてください。
// テストには、要素の追加、削除、先頭要素の参照など、主要な機能に対するケースを含めてください。

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        match self.items.len() {
            0 => None,
            _ => Some(self.items.remove(0)),
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.items.len() {
            0 => None,
            _ => Some(&self.items[0]),
        }
    }
}

fn main() {
    let mut queue1 = Queue::new();
    queue1.enqueue(1);
    queue1.enqueue(2);
    queue1.enqueue(3);
    println!("{:?}", queue1.peek());
    println!("{:?}", queue1.dequeue());
    println!("{:?}", queue1.dequeue());
    println!("{:?}", queue1.dequeue());
    println!("{:?}", queue1.dequeue());

    let mut queue2 = Queue::new();
    queue2.enqueue("a");
    queue2.enqueue("b");
    queue2.enqueue("c");
    println!("{:?}", queue2.peek());
    println!("{:?}", queue2.dequeue());
    println!("{:?}", queue2.dequeue());
    println!("{:?}", queue2.dequeue());
    println!("{:?}", queue2.dequeue());
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_operations() {
        let mut queue1 = Queue::new();
        queue1.enqueue(1);
        queue1.enqueue(2);
        queue1.enqueue(3);
        assert_eq!(queue1.peek(), Some(&1));
        assert_eq!(queue1.dequeue(), Some(1));
        assert_eq!(queue1.dequeue(), Some(2));
        assert_eq!(queue1.dequeue(), Some(3));
        assert_eq!(queue1.dequeue(), None);

        let mut queue2 = Queue::new();
        queue2.enqueue("a");
        queue2.enqueue("b");
        queue2.enqueue("c");
        assert_eq!(queue2.peek(), Some(&"a"));
        assert_eq!(queue2.dequeue(), Some("a"));
        assert_eq!(queue2.dequeue(), Some("b"));
        assert_eq!(queue2.dequeue(), Some("c"));
        assert_eq!(queue2.dequeue(), None);
    }
}
