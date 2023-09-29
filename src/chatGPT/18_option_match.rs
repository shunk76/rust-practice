fn add_options(option1: Option<i32>, option2: Option<i32>, option3: Option<i32>) -> Option<i32> {
    let options = [option1, option2, option3];
    let mut total = 0;

    for option in options.iter() {
        match option {
            Some(n) => total += n,
            None => return None,
        }
    }

    Some(total)
}

fn main() {
    let a = Some(2);
    let b = Some(3);
    let c = Some(4);
    let d = None;

    println!("{:?}", add_options(a, b, c)); // 出力は Some(9) であるべき
    println!("{:?}", add_options(a, b, d)); // 出力は None であるべき
}
