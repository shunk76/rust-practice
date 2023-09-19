// P.269

fn main() {
    let zip_code = "105-0011";

    // スライスで分割
    println!("-- スライス --");
    println!("前半: {}", &zip_code[..3]);
    println!("後半: {}", &zip_code[4..]);

    // split_at で分割
    println!("-- split_at --");
    // タプル型に分割代入
    let (zip1, zip2) = zip_code.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    // split_off で分割
    println!("-- split_off --");
    let mut zip1 = String::from(zip_code);
    // zip1 に 3バイト(105) を残し、残りの部分(-0011) を zip2 に代入
    let mut zip2 = zip1.split_off(3);
    // zip2 に 1バイト(-) を残し、残りの部分(0011) を zip3 に代入
    let zip3 = zip2.split_off(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    // split で分割
    println!("-- split --");
    let zip_a: Vec<&str> = zip_code.split('-').collect();
    println!("前半: {}", zip_a[0]);
    println!("後半: {}", zip_a[1]);
}
