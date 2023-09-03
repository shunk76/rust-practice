// P.102

fn main() {
    let mut v = 10;

    set_value(&mut v);

    println!("{}", v);
}

fn set_value(arg: &mut u32) {
    // デリファレンス: arg によって参照される値にアクセス
    *arg = 100;
}
