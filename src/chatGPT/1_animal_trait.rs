// 問題1

// 次の仕様を持つ簡単な"動物"システムを作成してください。

// Animalという名前のtraitを作成し、このtraitにはsound(&self)というメソッドを定義してください。このメソッドは文字列を返すものとします。
// DogとCatという名前のstructを作成してください。
// これらのstructに対して、Animal traitを実装してください。
// Dogのsoundメソッドは"woof"を返すようにしてください。
// Catのsoundメソッドは"meow"を返すようにしてください。
// main関数内でDogとCatのインスタンスを作成し、それぞれのsoundメソッドを呼び出して結果を出力してください。

trait Animal {
    fn sound(&self) -> String;
}

struct Dog {}

impl Animal for Dog {
    fn sound(&self) -> String {
        "woof".to_string()
    }
}

struct Cat {}

impl Animal for Cat {
    fn sound(&self) -> String {
        "meow".to_string()
    }
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};

    println!("{}", dog.sound());
    println!("{}", cat.sound());
}
