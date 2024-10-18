mod steg;
mod utils;

#[cfg(test)]
mod tests {
    use image::{DynamicImage, GenericImageView, Pixel, RgbImage, Rgba};

    use crate::utils::{bits, fs};
    use crate::steg::{hide, reveal};

    const IMAGE: &str = "mock/saitama.png";

    fn create_mock_image() -> (DynamicImage, Vec<[u8; 4]>) {
        let mut img = RgbImage::new(3, 3);

        let vals: Vec<[u8; 4]> = vec![
            [255, 0, 0, 255], [255, 255, 0, 255], [255, 0, 255, 255],
            [255, 0, 0, 255], [0, 255, 0, 255],   [0, 0, 255, 255],
            [0, 0, 255, 255], [0, 255, 0, 255], [150, 150, 255, 255]
        ];

        for i in 0..3 {
            for j in 0..3 {
                let pixel = Rgba(*vals.get((i * 3) + j).unwrap());
                img.put_pixel(i as u32, j as u32, pixel.to_rgb());
            }
        }

        (DynamicImage::ImageRgb8(img), vals)
    }

    #[test]
    fn check_file_exists() {
        assert!(fs::check_file_exists(IMAGE));
    }

    #[test]
    fn check_file_not_exists() {
        assert_eq!(fs::check_file_exists("mock/saitama1.png"), false);
    }

    #[test]
    fn test_image_load() {
        let img = fs::load_image(IMAGE).unwrap();

        assert_eq!(img.dimensions(), (50, 50));
    }

    #[test]
    fn test_image_save() {
        let img = fs::load_image(IMAGE).unwrap();
        let save_name = "test.png";
        img.save(save_name).unwrap();

        assert!(fs::check_file_exists(save_name));

        std::fs::remove_file(save_name).unwrap();
        assert!(!fs::check_file_exists(save_name));
    }

    #[test]
    fn test_char2bit() {
        let numbers: Vec<u8> = vec![0, 1, 14, 23, 255];
        let numbers_bits: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        for (i, n) in numbers.iter().enumerate() {
            assert_eq!(bits::u8_to_bit(*n), numbers_bits[i]);
        }
    }

    #[test]
    fn test_vec8_to_str() {
        let numbers_bits: Vec<u8> = vec![
            0, 0, 1, 0, 0, 0, 0, 1,
            0, 1, 1, 0, 0, 0, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 1, 1, 1, 1, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 0
        ];
        let chars = "!abz\u{80}".to_string();
        assert_eq!(bits::vec8_to_str(&numbers_bits), chars);
    }

    #[test]
    fn test_bytes_to_bits() {
        let numbers_bits: Vec<u8> = vec![
            0, 0, 1, 0, 0, 0, 0, 1,
            0, 1, 1, 0, 0, 0, 0, 1,
            0, 1, 1, 0, 0, 0, 1, 0,
            0, 1, 1, 1, 1, 0, 1, 0,
        ];
        let chars = b"!abz";

        assert_eq!(bits::bytes_to_bits(chars), numbers_bits);
    }

    #[test]
    fn test_hide_reveal() {
        let img = fs::load_image("mock/original.png").expect("READ ERROR");
    
        let data = bits::bytes_to_bits(b"ola mundo!");
        let embed_text = hide(&img, &data).expect("EMBEDDING ERROR");
        let extracted_text = reveal(&embed_text, 10);

        assert_eq!(extracted_text.unwrap(), "ola mundo!\0");
    }

    #[test]
    fn test_hide() {
        let (orig, _) = create_mock_image();

        let data = bits::bytes_to_bits(b"oi");
        let embed = hide(&orig, &data).expect("EMBEDDING ERROR");

        let expected: Vec<[u8; 4]> = vec![
            [254, 1, 0, 255], [254, 255, 0, 255], [255, 1, 255, 255],
            [255, 1, 0, 255], [1, 254, 0, 255],   [1, 0, 255, 255],
            [1, 0, 255, 255], [1, 254, 0, 255], [150, 150, 255, 255]
        ];


        for i in 0..3 {
            for j in 0..3 {
                let pixel = embed.get_pixel(i, j);
                let index: usize = ((i*3) + j) as usize;

                let original_val = expected[index];
                let embed_val = pixel.0;
                
                assert_eq!(original_val, embed_val);
            }
        }
        
    }

}