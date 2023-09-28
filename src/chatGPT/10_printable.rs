// `Printable`というtraitを定義し、それに`print_info`というメソッドを持たせる
// `Book`と`Magazine`という2つのstructを定義する
// 各structで`Printable` traitを実装し、`print_info`メソッドをオーバーライドする
// `print_info`はその本や雑誌の情報（例：タイトル、著者など）を文字列で出力する
// `main`関数で`Book`と`Magazine`オブジェクトを生成し、それぞれの情報を出力する

trait Printable {
    fn print_info(&self);
}

struct Book {
    title: String,
    author: String,
    price: u32,
}

struct Magazine {
    title: String,
    publisher: String,
    price: u32,
}

impl Book {
    fn new(title: &str, author: &str, price: u32) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            price,
        }
    }
}

impl Magazine {
    fn new(title: &str, publisher: &str, price: u32) -> Self {
        Magazine {
            title: title.to_string(),
            publisher: publisher.to_string(),
            price,
        }
    }
}

impl Printable for Book {
    fn print_info(&self) {
        println!("『{}』{} 著 ({}円)", self.title, self.author, self.price)
    }
}

impl Printable for Magazine {
    fn print_info(&self) {
        println!("『{}』{} ({}円)", self.title, self.publisher, self.price)
    }
}

fn main() {
    let book = Book::new("走れメロス", "太宰治", 550);
    let magazine = Magazine::new("少年ジャンプ", "集英社", 330);

    book.print_info();
    magazine.print_info();
}
