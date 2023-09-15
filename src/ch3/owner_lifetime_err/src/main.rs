// P.219

fn main() {
    let m = get_message();
    println!("{}", m);
}

fn get_message() -> &str {
    let msg = String::from("Hello");
    return &msg;
}
