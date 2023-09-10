// P.191

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("find_file (path) (keyword)");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];

    let target = path::PathBuf::from(target_dir);
    find_file(&target, keyword);
}

fn find_file(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("存在しないパス");

    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        if path.is_dir() {
            find_file(&path, keyword);
            continue;
        }

        let fname = path.file_name().unwrap().to_string_lossy();
        // println!("{}", fname);

        if None == fname.find(keyword) {
            continue;
        }

        println!("FIND {} ! ", path.to_string_lossy());
    }
}
