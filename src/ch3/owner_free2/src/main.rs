// P.209

fn main() {
    {
        let s1 = String::from("hello");
        let s3 = String::from("world");

        {
            let s2 = s1;
            println!("{}", s2);
            // s2 のスコープはここまで
        }

        println!("{}", s3);
        // s3 のスコープはここまで
    }
}
