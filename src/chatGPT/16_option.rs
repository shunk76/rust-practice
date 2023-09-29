// 平方根を求める関数を実装してください。
// 入力が負の場合はNoneを返し、それ以外の場合はSome(f64)を返します。
fn sqrt(number: f64) -> Option<f64> {
    match number {
        n if n < 0.0 => None,
        _ => Some(number.sqrt()),
    }
}

fn main() {
    let numbers = vec![16.0, 4.0, -1.0];

    for number in numbers {
        match sqrt(number) {
            Some(result) => println!("The square root of {} is {}", number, result),
            None => println!("Cannot calculate the square root of {}", number),
        }
    }
}
