// P.221

fn main() {
    let mut msg = String::from("Hello, World");
    println!("{}", msg);
    add_quote(&mut msg);
    println!("{}", msg);
}

fn add_quote(msg: &mut String) {
    msg.insert(0, '『');
    msg.push('』');
}
