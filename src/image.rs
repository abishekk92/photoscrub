use exif;
use exif::experimental::Writer;
use img_parts::jpeg::Jpeg;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::exif_utils;

pub struct Image {
    pub raw: Jpeg,
    pub exif: ImageMetadata,
}

pub struct ImageMetadata {
    pub raw: exif::Exif,
}

impl Image {
    pub fn from_file(filename: &PathBuf) -> Self {
        let raw_exif = exif_utils::read_exif(&filename).unwrap();
        let input = fs::read(filename).unwrap();
        let jpeg = Jpeg::from_bytes(input.into()).unwrap();
        return Image {
            exif: ImageMetadata { raw: raw_exif },
            raw: jpeg,
        };
    }

    pub fn write(self: &Self, output_file: &PathBuf) {
        let mut output = File::create(output_file).expect("Can't create file");
        let exif = self.exif.to_bytes();
        output.write_all(&exif).expect("Failed writing")
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

    pub fn to_bytes(self: &Self) -> Vec<u8> {
        let mut writer = Writer::new();
        let mut buf = std::io::Cursor::new(Vec::new());
        for f in self.raw.fields() {
            writer.push_field(&f);
        }
        writer.write(&mut buf, false).expect("Unable to write");
        return buf.into_inner();
    }
}

mod test {
    use super::*;
    #[test]
    #[ignore]
    fn test_load_read() {
        let image_path = PathBuf::from("test_images/meme.jpeg");
        let image = Image::from_file(&image_path);
        image.exif.print(true);
        println!("{:?}", image.raw)
    }

    #[test]
    #[ignore]
    fn test_to_bytes() {
        let image_path = PathBuf::from("test_images/meme.jpeg");
        let image = Image::from_file(&image_path);
        let bytes = image.exif.to_bytes();
        println!("{:?}", bytes);
    }

    #[test]
    fn test_write_image() {
        let image_path = PathBuf::from("test_images/meme.jpeg");
        let image = Image::from_file(&image_path);
        image.write(&PathBuf::from("out.Jpeg"));
    }
}
