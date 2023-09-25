// Result型を使用して、次の要件に沿った簡単なプログラムを作成してください。

// Personという名前の構造体を作成します。フィールドにはname: Stringとage: u8を持つようにしてください。
// Person構造体には、newという関連関数（static method）を作成します。この関数はname: &strとage: u8を引数として受け取ります。
// new関数内で、ageが18歳以上60歳以下である場合にはOk(Person)を、それ以外の場合にはErrとなるようなエラーメッセージを返すようにしてください。

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Result<Person, String> {
        if age < 18 || 60 < age {
            let err_msg = format!(
                "{}: {}歳 年齢が不適切です。18歳以上60歳以下である必要があります。",
                name, age
            );
            return Err(err_msg);
        }

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let alice = Person::new("Alice", 17);
    match alice {
        Ok(alice) => println!("{}: {}歳", alice.name, alice.age),
        Err(err) => println!("{}", err),
    }

    let bob = Person::new("Bob", 30);
    match bob {
        Ok(bob) => println!("{} {}", bob.name, bob.age),
        Err(err) => println!("{}", err),
    }
}
