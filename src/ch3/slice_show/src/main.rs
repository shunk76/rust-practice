// P.235

fn main() {
    let a = [0, 1, 2, 3, 4, 5];
    let a_slice = &a[0..3];
    // a_slice = &a[..3]; とも書ける
    println!("{:?}", a_slice);
    println!("{:?}", &a[3..5]);
    println!("{:?}", &a[4..6]);
    // &a[4..] とも書ける
    println!("{:?}", &a[..]);
}
