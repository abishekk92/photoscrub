use exif;
use img_parts::jpeg::Jpeg;
use std::fs;
use std::path::PathBuf;

use crate::exif_utils;

pub struct Image {
    pub raw: Jpeg,
    pub metadata: ImageMetadata,
}

pub struct ImageMetadata {
    pub raw: exif::Exif,
}

impl Image {
    pub fn from_file(filename: &PathBuf) -> Self {
        let raw_exif = exif_utils::read_exif(&filename).unwrap();
        let metadata = ImageMetadata { raw: raw_exif };
        let input = fs::read(filename).unwrap();
        let jpeg = Jpeg::from_bytes(input.into()).unwrap();
        return Image {
            metadata: metadata,
            raw: jpeg,
        };
    }
}

impl ImageMetadata {
    pub fn print(self: &Self, show: bool) {
        for f in self.raw.fields() {
            if show {
                println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
            } else {
                println!("{} {} ******", f.tag, f.ifd_num);
            }
        }
    }

    // pub fn to_bytes(self: &Self) {
    // }
}

#[test]
fn test_image() {
    let image_path = PathBuf::from("test_images/meme.jpg");
    let image = Image::from_file(&image_path);
    image.metadata.print(true);
    println!("{:?}", image.raw)
}
