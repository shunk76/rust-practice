// 問題2

// 簡単なショッピングカートを模倣するRustのプログラムを作成してください。以下の要件を満たしてください。

// Itemという名前のstructを作成してください。このstructはname: Stringとprice: f64というフィールドを持つとします。
// Cartという名前のstructを作成してください。このstructはitems: Vec<Item>というフィールドを持つとします。
// Cartにはadd_item(&mut self, item: Item)とtotal_price(&self) -> f64という2つのメソッドを実装してください。
// add_itemは、引数で与えられたItemをCartのitemsに追加します。
// total_priceは、カート内の全てのアイテムの価格の合計を返します。
// main関数内でCartのインスタンスを作成し、いくつかのItemを追加した後で、total_priceメソッドを呼び出してカートの合計価格を出力してください。

struct Item {
    name: String,
    price: f64,
}

struct Cart {
    items: Vec<Item>,
}

impl Cart {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        println!("アイテム {} がカートに追加されました。", item.name);
        self.items.push(item);
    }

    fn total_price(&self) -> f64 {
        self.items.iter().map(|item| item.price).sum()
    }
}

fn main() {
    let mut cart = Cart::new();

    let item1 = Item {
        name: "商品 1".to_string(),
        price: 1000.0,
    };
    let item2 = Item {
        name: "商品 2".to_string(),
        price: 1500.0,
    };
    let item3 = Item {
        name: "商品 3".to_string(),
        price: 2000.0,
    };

    cart.add_item(item1);
    cart.add_item(item2);
    cart.add_item(item3);

    println!("合計金額: {:.0}円", cart.total_price());
}
