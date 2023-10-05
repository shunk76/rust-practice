// Stack<T>という名前のジェネリックな構造体を定義してください。
// この構造体は、ジェネリックな型Tの要素を保持するベクターを内部に持つものとします。
// 以下のメソッドをこの構造体に実装してください。
// new() -> Self: 空のStackを作成します。
// push(item: T): 要素をスタックにプッシュします。
// pop() -> Option<T>: スタックから要素をポップします。スタックが空の場合はNoneを返します。

struct Stack<T> {
    v: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { v: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.v.push(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
}
