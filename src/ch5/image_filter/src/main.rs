// P.381

use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] image_filter imagefile");
        return;
    }

    let in_file = args[1].clone();
    let out_file = format!("{}-out.jpg", in_file);
    println!("in_file={}", in_file);
    println!("out_file={}", out_file);

    let mut img = image::open(in_file).expect("画像が読めません");

    let (w, h) = img.dimensions();

    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([255 - c[0], 255 - c[1], 255 - c[2], c[3]]);
            img.put_pixel(x, y, c);
        }
    }

    img.save(out_file).unwrap();
}
