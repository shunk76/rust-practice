// P.353

mod a {
    pub mod b {
        pub mod c {
            pub fn print() {
                println!("a::b::c::print");
            }
        }
    }

    pub mod d {
        pub mod e {
            pub fn print() {
                println!("a::d::e::print");
            }
        }

        pub mod f {
            pub fn print() {
                // 想定的に関数を呼ぶ
                super::e::print();
                super::super::b::c::print();
            }
        }
    }
}

fn main() {
    a::b::c::print();
    a::d::e::print();
    a::d::f::print();

    // ルートから指定
    crate::a::d::f::print();
}
