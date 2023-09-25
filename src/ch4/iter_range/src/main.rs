// P.321

fn main() {
    for i in 1..=7 {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }

    let v: Vec<i32> = (1..=7).filter(|&i| i % 2 == 1).collect();
    println!("{:?}", v)
}
