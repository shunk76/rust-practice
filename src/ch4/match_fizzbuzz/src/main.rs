// P.339

fn main() {
    for i in 1..=100 {
        // タプル
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (_, 0) => println!("Fizz"),
            (0, _) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
