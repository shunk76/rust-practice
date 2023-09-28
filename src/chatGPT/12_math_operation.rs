// 1. `MathOperation`という名前のenumを定義し、Add, Subtract, Multiply, Divideという値を持たせる。
// 2. `Calculator`という名前のstructを定義する。
// 3. `Calculator`には`calculate`というメソッドを持たせ、MathOperationを引数に取り、計算結果を返す。除算の場合は0で除算する場合にはエラーを返すように。
// 4. `calculate`メソッドの戻り値は`Result<f64, String>`とする。

enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Calculator;

impl Calculator {
    fn calculate(operation: MathOperation, a: f64, b: f64) -> Result<f64, String> {
        match operation {
            MathOperation::Add => Ok(a + b),
            MathOperation::Subtract => Ok(a - b),
            MathOperation::Multiply => Ok(a * b),
            MathOperation::Divide => {
                if b == 0.0 {
                    Err("0 では割れんのや、すまんのう".to_string())
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
        Err(str) => println!("{}", str),
    }
}

fn main() {
    print_result(Calculator::calculate(MathOperation::Add, 10.0, 15.2));
    print_result(Calculator::calculate(MathOperation::Subtract, 10.0, 15.2));
    print_result(Calculator::calculate(MathOperation::Multiply, 10.0, 15.2));
    print_result(Calculator::calculate(MathOperation::Divide, 10.0, 15.2));
    print_result(Calculator::calculate(MathOperation::Divide, 10.0, 0.0));
}
