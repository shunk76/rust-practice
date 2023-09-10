// P.189

fn main() {
    for i in 2..=20 {
        println!("{}", fib(i))
    }
}

// n番目のフィボナッチ数を求める
fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    fib(n - 2) + fib(n - 1)
}
