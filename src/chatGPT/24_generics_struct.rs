// Stackという名前のジェネリクス構造体を作成してください。この構造体はVec<T>を一つのフィールドとして保持します。
// Stack<T>に以下のメソッドを実装してください：
// new() - 新しい空のスタックを作成します。
// push(item: T) - アイテムをスタックにプッシュします。
// pop() -> Option<T> - スタックのトップのアイテムをポップ（取り出し）します。スタックが空の場合はNoneを返します。

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push("a");
    stack.push("b");
    stack.push("c");
    stack.push("d");
    println!("{:?}", stack);

    for _ in 0..(stack.items.len() + 1) {
        println!("{:?}", stack.pop());
    }
}
