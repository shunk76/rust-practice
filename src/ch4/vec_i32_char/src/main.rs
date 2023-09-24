// P.312

fn main() {
    let mut vec1: Vec<i32> = Vec::<i32>::new();
    vec1.push(10);
    vec1.push(20);
    vec1.push(30);
    vec1.pop();
    for i in vec1.iter() {
        println!("{}", i);
    }

    let mut vec2: Vec<char> = Vec::<char>::new();
    vec2.push('a');
    vec2.push('b');
    vec2.push('c');
    vec2.pop();
    for i in vec2.iter() {
        println!("{}", i);
    }
}
