// safe_divide という関数を作成してください。
// この関数は2つの f64 数値を受け取り、Result<f64, String> を返します。
// 関数は、第2引数が 0.0 でない場合は除算を行い、Ok(result) を返します。
// 第2引数が 0.0 の場合は、Err(String) を返し、エラーメッセージを含めてください。

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("エラーやし。0 で割れんし。".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", safe_divide(100.0, 2.5));
    println!("{:?}", safe_divide(100.0, 0.0));
}
