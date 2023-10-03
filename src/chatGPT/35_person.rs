struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn introduce(&self) {
        println!("私の名前は{}です。{}歳です。", self.name, self.age)
    }
}

fn main() {
    let mike = Person::new("マイク", 30);
    mike.introduce();
}
