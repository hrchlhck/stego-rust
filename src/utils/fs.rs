use image::{io::Reader as ImageReader, DynamicImage, ImageError};
use std::path::Path;

pub fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    let res = match ImageReader::open(path) {
        Ok(img) => img.decode(),
        Err(e) => Err(ImageError::IoError(e))
    };

    res
}

pub fn save_image(image: DynamicImage, path: &str) -> bool {
    match image.save(path) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn check_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

