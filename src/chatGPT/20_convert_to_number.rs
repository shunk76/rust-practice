fn convert_to_number(s: &str) -> Result<i32, &'static str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Not a valid number"),
    }
}

fn main() {
    println!("{:?}", convert_all_to_numbers(vec!["1", "2", "3"]));
    println!("{:?}", convert_all_to_numbers(vec!["1", "b", "3"]));
}

fn convert_all_to_numbers(v: Vec<&str>) -> Result<Vec<i32>, &str> {
    v.iter().map(|s| convert_to_number(s)).collect()
}
