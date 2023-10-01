fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("0 では割れんのや".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn print_result(result: Result<f64, String>) {
    match result {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    print_result(divide(10.0, 2.0));
    print_result(divide(10.0, 0.0));
}
