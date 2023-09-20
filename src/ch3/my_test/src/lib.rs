// P.284

// ライブラリとしてプロジェクトを作る
// cargo new my_test --lib
// cargo test

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// アトリビュート
#[cfg(test)]

// モジュール
mod tests {
    use super::*;

    // cargo test 時にこの宣言がある関数が実行される
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
