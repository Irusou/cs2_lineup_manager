use std::{io::ErrorKind, path::Path};

use image::{ColorType, ImageFormat, ImageResult, save_buffer_with_format};
use uuid::Uuid;

use crate::domain::lineup::Image;

impl Image {
    pub fn save_image(
        width: u32,
        height: u32,
        buf: &[u8],
        folder: &Path,
        format: ImageFormat,
    ) -> ImageResult<String> {
        let usize_w = width as usize;
        let usize_h = height as usize;

        let Some(buf_size) = usize_w
            .checked_mul(usize_h)
            .and_then(|bs| bs.checked_mul(4))
        else {
            return Err(image::ImageError::IoError(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Failed to parse buffer size",
            )));
        };

        if buf.len() != buf_size {
            return Err(image::ImageError::IoError(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid buffer size",
            )));
        }

        // gerar extensão suportada
        let extension = match format {
            ImageFormat::Png => Ok(".png"),
            ImageFormat::Jpeg => Ok(".jpeg"),
            ImageFormat::WebP => Ok(".webp"),
            _ => Err(image::ImageError::IoError(std::io::Error::new(
                ErrorKind::InvalidInput,
                "File format not supported",
            ))),
        }?;

        // criar folder
        std::fs::create_dir_all(folder).map_err(image::ImageError::IoError)?;

        // gerar filename
        let uuid = Uuid::new_v4();
        let filename = format!("{uuid}{extension}");

        // build full path folder+filename
        let full_path = folder.join(&filename);

        // save full path
        save_buffer_with_format(full_path, buf, width, height, ColorType::Rgba8, format)?;

        // return Ok(filename)
        Ok(filename)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::tempdir;

    // 2x2 image: red, green, blue, white (all opaque) = 16 bytes
    fn colored_2x2() -> Vec<u8> {
        vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 255, 255, // white
        ]
    }

    #[test]
    fn saves_png_into_a_folder_that_does_not_exist_yet() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images"); // not created yet

        let name = Image::save_image(2, 2, &colored_2x2(), &folder, ImageFormat::Png)
            .expect("save should succeed");

        assert!(name.ends_with(".png"));
        assert!(folder.join(&name).exists());
    }

    #[test]
    fn saved_file_is_a_valid_image_with_the_right_content() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");

        let name = Image::save_image(2, 2, &colored_2x2(), &folder, ImageFormat::Png).unwrap();

        let img = image::open(folder.join(&name)).unwrap().to_rgba8();
        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);
        assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255]); // top-left: red
        assert_eq!(img.get_pixel(1, 1).0, [255, 255, 255, 255]); // bottom-right: white
    }

    #[test]
    fn non_square_image_with_correct_buffer_saves() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");
        let buf = vec![200u8; 3 * 2 * 4]; // 24 bytes

        let result = Image::save_image(3, 2, &buf, &folder, ImageFormat::Png);

        assert!(result.is_ok());
    }

    #[test]
    fn non_square_image_with_one_byte_missing_fails() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");
        let buf = vec![200u8; 3 * 2 * 4 - 1]; // 23 bytes

        let result = Image::save_image(3, 2, &buf, &folder, ImageFormat::Png);

        assert!(result.is_err());
    }

    #[test]
    fn wrong_buffer_size_returns_our_error_and_touches_no_disk() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");
        let buf = vec![0u8; 15]; // a 2x2 image needs 16

        let err = Image::save_image(2, 2, &buf, &folder, ImageFormat::Png).unwrap_err();

        assert!(err.to_string().contains("Invalid buffer size"));
        assert!(!folder.exists(), "folder must not be created on bad input");
    }

    #[test]
    fn overflowing_dimensions_return_our_error() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");

        let err =
            Image::save_image(u32::MAX, u32::MAX, &[], &folder, ImageFormat::Png).unwrap_err();

        assert!(err.to_string().contains("Failed to parse buffer size"));
        assert!(!folder.exists());
    }

    #[test]
    fn unsupported_format_is_rejected_before_touching_disk() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");

        let err = Image::save_image(2, 2, &colored_2x2(), &folder, ImageFormat::Gif).unwrap_err();

        assert!(err.to_string().contains("File format not supported"));
        assert!(
            !folder.exists(),
            "folder must not be created for a bad format"
        );
    }

    #[test]
    fn two_saves_get_different_filenames() {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().join("images");

        let a = Image::save_image(2, 2, &colored_2x2(), &folder, ImageFormat::Png).unwrap();
        let b = Image::save_image(2, 2, &colored_2x2(), &folder, ImageFormat::Png).unwrap();

        assert_ne!(a, b);
        assert!(folder.join(&a).exists());
        assert!(folder.join(&b).exists());
    }

    #[test]
    fn folder_path_that_is_actually_a_file_returns_an_error() {
        let tmp = tempdir().unwrap();
        let not_a_folder = tmp.path().join("im_a_file");
        std::fs::write(&not_a_folder, b"hello").unwrap();

        let result = Image::save_image(2, 2, &colored_2x2(), &not_a_folder, ImageFormat::Png);

        assert!(result.is_err());
    }
}
