use num_bigint::BigInt;
use std::time::Instant;

fn main() {
    let v = BigInt::from(1234);
    let result = v.pow(5678);
    println!("{}, {}桁", result, result.to_string().len());

    let start = Instant::now();
    collatz(result);
    let duration = start.elapsed();

    println!("時間: {:?}", duration);
}

fn collatz(mut n: BigInt) {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let three = BigInt::from(3);
    let mut odd = 0;
    let mut even = 0;

    while n > one.clone() {
        if &n % &two == one.clone() {
            odd += 1;
            n = n * &three + &one;
        } else {
            even += 1;
            n = n / &two;
        }
    }

    println!("{}, 奇数: {}回, 偶数: {}回", n, odd, even)
}
