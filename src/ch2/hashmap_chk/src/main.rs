// P.161

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // map.insert("D", 40);

    match map.get("D") {
        None => println!("Dは存在しない {:?}", map.get("D")),
        Some(v) => println!("D = {}", v),
    }
}
