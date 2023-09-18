// P.268

fn main() {
    {
        // シャドーイングを使わない
        let mut v = 300;
        v = v + 5;
        println!("{}", v)
    }

    {
        // シャドーイングを使う
        let v = 300;
        let v = v + 5;
        println!("{}", v)
    }
}
