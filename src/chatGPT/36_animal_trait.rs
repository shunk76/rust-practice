trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!")
    }
}

fn make_animal_sound<A: Animal>(animal: A) {
    animal.make_sound();
}

fn main() {
    make_animal_sound(Dog);
    make_animal_sound(Cat);
}
