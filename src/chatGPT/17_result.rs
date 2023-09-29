// 与えられた文字列が整数に変換可能ならその整数を2倍にして返す関数double_numberを書いてください。
// 文字列が整数に変換できない場合は、適切なエラーメッセージとともにErrを返してください。
// いくつかのテストケースをmain関数内で実行してみてください。

fn double_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(v) => Ok(v * 2),
        Err(_) => Err("数値に変換できない文字です。".to_string()),
    }
}

fn print_result(result: Result<i32, String>) {
    match result {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    print_result(double_number("15"));
    print_result(double_number("abc"));
}
