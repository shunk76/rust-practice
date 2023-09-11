// P.194
use std::fs;

fn main() {
    let files = fs::read_dir("../").expect("不正なパスです");

    for ent in files {
        let entry = ent.unwrap();
        let path = entry.path();
        let fname = path.to_str().unwrap_or("不正なファイル名です");
        println!("{}", fname);
    }
}
