// 問題：
// i32型のベクター（Vec<i32>）を引数として受け取り、その要素の合計を返す関数sum_elementsを作成してください。
// さらに、この関数を高階関数として実装し、引数として別の関数（クロージャも可）を受け取れるようにしてください。
// この高階関数は、各要素に適用する前に引数として受け取った関数を要素に適用します。
// たとえば、引数として2倍する関数を受け取った場合、元のベクターの各要素を2倍してから合計を返すようにします。

fn sum_elements<F>(items: Vec<i32>, mut closure: F) -> i32
where
    F: FnMut(i32) -> i32,
{
    items.into_iter().map(|x| closure(x)).fold(0, |a, b| a + b)
}

fn main() {
    println!("{}", sum_elements(vec![1, 2, 3, 4, 5], |x| x * 2))
}
