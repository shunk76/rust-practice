// fn main() {
//     for i in 1..10 {
//         for j in 1..10 {
//             print!("{:3}", i * j);
//             if j < 9 {
//                 print!(",");
//             }
//         }
//         println!("");
//     }
// }

fn main() {
    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", s);
    }
}
