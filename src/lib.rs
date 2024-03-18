mod steg;
mod utils;

#[cfg(test)]
mod tests {
    use image::GenericImageView;
    use crate::utils::{fs, bits};
    const IMAGE: &str = "mock/saitama.png";

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
}