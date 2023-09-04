// P.103

fn main() {
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }

    println!("{}", sum);
}

// fn main() {
//     // 配列(固定長)を利用する
//     let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let mut total = 0;

//     for i in nums {
//         total += i;
//     }

//     println!("{}", total);
// }

// fn main() {
//     // ベクター(可変長)を利用する
//     let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let mut total = 0;

//     for i in nums {
//         total += i;
//     }

//     println!("{}", total);
// }
