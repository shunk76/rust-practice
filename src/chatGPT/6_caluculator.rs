// Rustのenumとmatch文を用いて、単純な四則演算の計算機を作成してください。
// enumを使ってOperationという名前で、足し算（Add）、引き算（Subtract）、
// 掛け算（Multiply）、割り算（Divide）の4つの操作を定義してください。
// また、関数calculateを作成し、この関数はOperation enumと2つのf64数字を引数に取ります。
// この関数は、指定された演算を行い、結果をResult<f64, String>で返してください。
// 割り算の際に、0で割ることがある場合は、Errでエラーメッセージを返してください。
// メイン関数で、いくつかのテストケースを用いてこのcalculate関数を試してください。

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
