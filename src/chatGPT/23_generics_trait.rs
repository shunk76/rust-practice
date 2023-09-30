// ジェネリクスとトレイトを用いて、各要素を2倍する関数を作成してみましょう。
// 関数は、任意の型 T の Vec<T> を引数にとります。
// T は std::ops::Mul と Copy のトレイトを実装している必要があります。
// 関数は、各要素を2倍した新しい Vec<T> を返します。

trait Calc<T: std::ops::Mul<Output = T> + Copy + From<u8>> {
    fn x2(&self, v: Vec<T>) -> Vec<T>;
}

struct Calculator;

impl<T: std::ops::Mul<Output = T> + Copy + From<u8>> Calc<T> for Calculator {
    fn x2(&self, v: Vec<T>) -> Vec<T> {
        v.iter().map(|&n| n * T::from(2)).collect()
    }
}

fn main() {
    let calc = Calculator;
    let v = vec![-1, 2, 3, 4, 5];
    println!("{:?}", calc.x2(v));
}
