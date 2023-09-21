// P.297

struct Person {
    age: i32,
    name: String,
}

impl Person {
    // 関連関数: self を引数に取らない Person::new() と書く
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let taro = Person::new("太郎".to_string(), 18);
    println!("{} さんは {} 歳", taro.name, taro.age)
}
