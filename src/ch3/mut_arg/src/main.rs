// P.222

fn main() {
    let mut v = 16;
    x2(&mut v);
    println!("{}", v);
}

// 参照渡し
fn x2(arg: &mut i32) {
    // デリファレンス(参照外し)
    *arg = *arg * 2;
}
