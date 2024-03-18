use image::{GenericImage, GenericImageView};

pub mod utils;

use crate::utils::{fs, bits};

fn hide() {
    let mut img: image::DynamicImage = fs::load_image("mock/teste.png").unwrap();
    let msg = b"abc";
    let ch_bit: Vec<u8> = bits::bytes_to_bits(msg);

    let mut x = 0;
    let mut y = 0;
    let mut channel = 0;
    let height = img.height();
    let width = img.width();
    for bit in ch_bit{
        if x == width {
            x = 0;
            y += 1;
        }

        if y == height {
            x = 0;
            y = 0;
            channel += 1;
        }

        if y == height && x == width && channel == 3 {
            println!("End Of Image (EOI)");
            break;
        }

        if x < width {
            let mut pixel = img.get_pixel(x, y);
    
            if bit == 1 {
                pixel.0[channel] |= 1;
            } else {
                pixel.0[channel] &= 0xFE;
            }
            img.put_pixel(x, y, pixel);
            x += 1;
        }
    }

    img.save("test.png").unwrap();
}

fn reveal(n: u32) {
    let img: image::DynamicImage = fs::load_image("test.png").unwrap();
    let mut res: Vec<u8> = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut channel = 0;
    let height = img.height();
    let width = img.width();
    for _ in 0..=n {
        if x == width {
            x = 0;
            y += 1;
        }

        if y == height {
            x = 0;
            y = 0;
            channel += 1;
        }

        if y == height && x == width && channel == 3 {
            println!("End Of Image (EOI)");
            break;
        }

        if x < width {
            res.push(img.get_pixel(x, y).0[channel] & 1);
            x += 1;
        }
    }
    println!("{}", bits::vec8_to_str(&res));
}

fn main() {
    hide();
    reveal(24);
}