// P.158

use std::collections::HashMap;

const V_DATA: &str = "C,C,A,A,A,A,B,B,A,C,B,C";

fn main() {
    let mut c_map = HashMap::new();

    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);

    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w] + 1);
    }

    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, c_map[k])
    }
}
