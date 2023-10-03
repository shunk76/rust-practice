fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("0 では割れんのよ".to_string())
    } else {
        Ok(a / b)
    }
}

fn find_even(numbers: Vec<i32>) -> Option<i32> {
    numbers.iter().find(|&n| n % 2 == 0).copied()
}

fn main() {
    match divide(20.0, 5.0) {
        Ok(v) => println!("{}", v),
        Err(m) => println!("{}", m),
    }

    match divide(20.0, 0.0) {
        Ok(v) => println!("{}", v),
        Err(m) => println!("{}", m),
    }

    match find_even(vec![1, 5, 6, 4, 3]) {
        Some(n) => println!("{}", n),
        None => println!("偶数はなかった"),
    }

    match find_even(vec![1, 3, 5, 7]) {
        Some(n) => println!("{}", n),
        None => println!("偶数はなかった"),
    }
}
