trait ToJson {
    fn to_json(&self) -> String;
}

struct User {
    username: String,
    email: String,
}

struct Product {
    name: String,
    price: f64,
}

// ここに ToJson トレイトを User と Product に対して実装してください。
// to_json メソッドは、各フィールドを JSON 形式（String）に変換して返します。
// 例：User の場合 {"username": "example", "email": "example@example.com"}

impl ToJson for User {
    fn to_json(&self) -> String {
        // ここにコードを書いてください。
        format!(
            "{{ \"username\": \"{}\", \"email\": \"{}\" }}",
            self.username, self.email
        )
    }
}

impl ToJson for Product {
    fn to_json(&self) -> String {
        // ここにコードを書いてください。
        format!(
            "{{ \"name\": \"{}\", \"price\": {} }}",
            self.name, self.price
        )
    }
}

fn main() {
    let user = User {
        username: "JohnDoe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    let product = Product {
        name: "Laptop".to_string(),
        price: 1000.0,
    };

    // 以下のコードで、それぞれのオブジェクトを JSON 形式に変換できるようにしてください。
    println!("User JSON: {}", user.to_json());
    println!("Product JSON: {}", product.to_json());
}
