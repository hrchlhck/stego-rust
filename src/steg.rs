use image::{DynamicImage, GenericImage, GenericImageView};

use crate::utils::bits::vec8_to_str;

pub fn hide(img: &DynamicImage, data: &Vec<u8>) -> Result<DynamicImage, String> {
    let height = img.height();
    let width = img.width();
    
    if height <= 0 || width <= 0 {
        return Err(format!("Error: The image dimensions must be greater than 0. Provided dimensions: ({width}, {height})"))
    }
    
    if data.len() == 0 {
        return Err(format!("Error: The data length must be greater than 0"))
    }

    let mut img_empty: DynamicImage = img.clone();
    let mut x = 0;
    let mut y = 0;
    let mut channel = 0;
    for bit in data {
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

        if channel == 3 {
            channel = 0;
        }

        
        if x < width {
            let mut pixel = img_empty.get_pixel(x, y);
            
            if *bit == 1 {
                pixel.0[channel] |= 0b00000001;
            } else {
                pixel.0[channel] &= 0b11111110;
            }

            img_empty.put_pixel(x, y, pixel);
            x += 1;
        }
    }

    Ok(DynamicImage::ImageRgb8(img_empty.into()))
}


pub fn reveal(img: &DynamicImage, n: u32) -> Result<String, String> {
    if n <= 0 {
        return Err("Error: n must be greater than 0".to_string())
    }

    let height = img.height();
    let width = img.width();

    if height <= 0 || width <= 0 {
        return Err(format!("Error: The image dimensions must be greater than 0. Provided dimensions: ({width}, {height})"))
    }

    let mut res: Vec<u8> = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut channel = 0;
    for _ in 0..=(n * 8) {
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

    Ok(vec8_to_str(&res))
}