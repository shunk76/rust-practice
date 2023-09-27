enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn calculate(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b == 0.0 {
                    Err("0 で割ることはできません".to_string())
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

fn print_result(result: Result<f64, String>) {
    match result {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    print_result(Operation::Add.calculate(1.0, 1.0));
    print_result(Operation::Subtract.calculate(1.0, 10.0));
    print_result(Operation::Multiply.calculate(2.0, 3.0));
    print_result(Operation::Divide.calculate(10.0, 2.0));
    print_result(Operation::Divide.calculate(10.0, 0.0));
}
