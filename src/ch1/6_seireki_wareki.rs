fn main() {
    let reiwa = 2019;
    let heisei = 1989;
    let showa = 1926;

    for y in 1926..2027 {
        print!("西暦{}年 = ", y);

        if y >= reiwa {
            if y == reiwa {
                println!("令和元年");
            } else {
                println!("令和{}年", y - reiwa + 1);
            }
        } else if y >= heisei {
            if y == heisei {
                println!("平成元年");
            } else {
                println!("平成{}年", y - heisei + 1);
            }
        } else if y >= showa {
            if y == showa {
                println!("昭和元年");
            } else {
                println!("昭和{}年", y - showa + 1);
            }
        }
    }
}
