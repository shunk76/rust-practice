// P.226

use chrono::{Datelike, Utc};

fn main() {
    let now = Utc::now();

    let year = now.year();
    let month = now.month();
    let day = now.day();

    println!("JP: {}/{}/{}", year, month, day);
    // 順番を指定
    println!("US: {1}/{2}/{0}", year, month, day);
    println!("UK: {2}/{1}/{0}", year, month, day);

    // 名前を指定
    println!("{Y}年{M}月{D}日", Y = year, M = month, D = day);

    // P.227
    // フォーマット
    // n桁で左寄せ {:<n}
    println!("_{:>5}_", 30);

    // n桁で右寄せ {:>n}
    println!("_{:>5}_", 30);

    // n桁で中央寄せ {:^n}
    println!("_{:^5}_", 3);

    // n桁で左寄せ(0埋め) {:<0n}
    println!("_{:<05}_", 30);

    // n桁で右寄せ(0埋め) {:>0n}
    println!("_{:>05}_", 30);

    // 2進数に変換
    println!("{:b}", 4);

    // 2進数で n桁 ゼロ埋め {:0nb}
    println!("{:08b}", 4);

    // 8進数に変換
    println!("{:o}", 438);

    // 16進数に変換
    println!("{:x}", 255);

    // 16進数で n桁 ゼロ埋め {:0nx}
    println!("{:04x}", 15);

    // 小数点以下を n桁 {:.n}
    println!("{:.2}", 3.1415);

    // 指数表現
    println!("{:e}", 12.34);

    // デバッグ出力
    println!("{:?}", [1, 2, 3]);

    // デバッグ出力(整形)
    println!("{:#?}", [1, 2]);

    // { を出力
    println!("{{");

    // } を出力
    println!("}}");

    // ポインタ
    println!("{:p}", "abc");

    println!("|{:>8}| #{:06x}", "red", 0xFF0000);
    println!("|{:>8}| #{:06x}", "green", 0x00FF00);
    println!("|{:>8}| #{:06x}", "blue", 0x0000FF);
    println!("|{:>8}| RGB{:?}", "yellow", (255, 255, 0));
}
