// P.338

fn main() {
    let i: u8 = 2u8;

    match i {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("..."),
    }
}
