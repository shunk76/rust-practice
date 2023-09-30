use std::cmp::PartialEq;
use std::convert::From;
use std::ops::Div;

fn divide<T: Div<Output = T> + PartialEq + From<u8>>(a: T, b: T) -> Result<T, String> {
    if b == T::from(0) {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn print_result<T: Div<Output = T> + PartialEq + std::fmt::Display + From<u8>>(
    result: Result<T, String>,
) {
    match result {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    print_result(divide(10, 2));
    print_result(divide(10.0, 2.0));
    print_result(divide(10, 0));
}
