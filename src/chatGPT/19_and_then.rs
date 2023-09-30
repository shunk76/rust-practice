fn find_even_sum(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .fold(Some(0), |acc, &x| {
            acc.and_then(|sum| if x % 2 == 0 { Some(sum + x) } else { Some(sum) })
        })
        .and_then(|sum| if sum == 0 { None } else { Some(sum) })
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    match find_even_sum(&numbers) {
        Some(sum) => println!("The sum of even numbers is {}", sum),
        None => println!("No even numbers found"),
    }
}
