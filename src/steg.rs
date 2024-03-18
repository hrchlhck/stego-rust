use image::{DynamicImage, GenericImageView};

pub fn hide(img: &DynamicImage, data: &Vec<u8>) -> DynamicImage {
    img.clone()
}