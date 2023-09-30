// この関数は、Vec<T>の各要素を表示する処理を書いてください。
// Hint: T は Debug trait を実装していると仮定しても大丈夫です。
fn print_elements<T: std::fmt::Debug>(elements: Vec<T>) {
    for e in elements.iter() {
        println!("{:?}", e);
    }
}

fn main() {
    let vec_of_ints = vec![1, 2, 3];
    let vec_of_strs = vec!["apple", "banana", "cherry"];

    print_elements(vec_of_ints);
    print_elements(vec_of_strs);
}
