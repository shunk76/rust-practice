// 1. `Animal` traitを定義し、`make_sound`メソッドを持たせる。
// 2. `Dog`, `Cat`, `Bird`という3つのstructを定義する。
// 3. 各structで`Animal` traitを実装し、`make_sound`メソッドをオーバーライドする。
// 4. `AnimalType`という名前で、犬、猫、鳥を表す列挙型（enum）を作成する。
// 5. `AnimalType` enumには`Dog(Dog)`, `Cat(Cat)`, `Bird(Bird)`という形で値を持たせる。
// 6. `AnimalType` enumに`Animal` traitを実装し、`make_sound`メソッドで内部のstructに応じた音を出力する。
// 7. `main`関数で、`AnimalType`の各バリアントを作成し、それぞれで`make_sound`メソッドを呼び出す。

trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;
struct Bird;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "わんわん".to_string()
    }
}
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "にゃーにゃー".to_string()
    }
}
impl Animal for Bird {
    fn make_sound(&self) -> String {
        "ちゅんちゅん".to_string()
    }
}

enum AnimalType {
    Dog(Dog),
    Cat(Cat),
    Bird(Bird),
}

impl Animal for AnimalType {
    fn make_sound(&self) -> String {
        match self {
            AnimalType::Dog(Dog) => Dog.make_sound(),
            AnimalType::Cat(Cat) => Cat.make_sound(),
            AnimalType::Bird(Bird) => Bird.make_sound(),
        }
    }
}

fn main() {
    println!("{}", AnimalType::Dog(Dog).make_sound());
    println!("{}", AnimalType::Cat(Cat).make_sound());
    println!("{}", AnimalType::Bird(Bird).make_sound());
}
