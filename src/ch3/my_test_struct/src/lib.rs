// P.290

// Debugトレイト: フォーマッタを利用して値を出力できる
// PartialEqトレイト: 構造体の各要素を比較できる
#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    // 外側の要素を利用
    use super::*;

    #[test]
    fn item_test() {
        let apple1 = GItem {
            name: String::from("りんご"),
            price: 2400,
        };

        let mut apple2 = GItem {
            name: "りんご".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        assert_eq!(apple1, apple2);
    }
}
