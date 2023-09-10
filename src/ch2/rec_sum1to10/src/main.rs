// P.188

fn main() {
    println!("{}", sum(10));
}

fn sum(n: i32) -> i32 {
    if n <= 0 {
        return n;
    }

    sum(n - 1) + n
}
