// structとtraitを使って、動物園の動物たちを表現するプログラムを書いてみてください。
// Animalというtraitを定義し、その中でmake_soundとnameというメソッドを定義してください。
// Dog、Cat、Birdというstructを作成して、それぞれにAnimal traitを実装してください。
// 例えば、Dogがmake_soundを呼ばれた場合は"woof"を返し、nameを呼ばれた場合は"Dog"を返すようにしてください。
// 最後に、Animal traitを実装した各動物のmake_soundとnameメソッドを呼び出すmain関数を書いてください。

trait Animal {
    fn make_sound(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn print_sound(&self) {
        println!("{}の鳴き声: {}", self.name(), self.make_sound());
    }
}

struct Dog;
struct Cat;
struct Bird;

impl Animal for Dog {
    fn make_sound(&self) -> &'static str {
        "わんわん"
    }
    fn name(&self) -> &'static str {
        "犬"
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &'static str {
        "にゃーにゃー"
    }
    fn name(&self) -> &'static str {
        "猫"
    }
}

impl Animal for Bird {
    fn make_sound(&self) -> &'static str {
        "ちゅんちゅん"
    }
    fn name(&self) -> &'static str {
        "鳥"
    }
}

fn main() {
    Dog.print_sound();
    Cat.print_sound();
    Bird.print_sound();
}

// 'static: ライフタイムがプログラム全体であることを示すライフタイム注釈。
