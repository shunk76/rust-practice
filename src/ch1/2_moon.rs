fn main() {
    let distance = 384400.0;
    let car = 80.0 * 24.0;
    let train = 300.0 * 24.0;

    println!("月まで車で{}日", distance / car);
    println!("月まで新幹線で{}日", distance / train);
}
