use exif;
use exif::experimental::Writer;
use img_parts::jpeg::Jpeg;
use img_parts::Bytes;
use img_parts::ImageEXIF;
use std::fs::{self, File};
use std::path::PathBuf;

pub struct Image {
    pub raw: Jpeg,
    pub exif: ImageMetadata,
}

pub struct ImageMetadata {
    pub raw: exif::Exif,
}

impl Image {
    pub fn from_file(filename: &PathBuf) -> Self {
        let raw_exif = ImageMetadata::from_file(filename).unwrap();
        let input = fs::read(filename).unwrap();
        let jpeg = Jpeg::from_bytes(input.into()).unwrap();
        Self {
            exif: raw_exif,
            raw: jpeg,
        }
    }
}

//TODO: Each of the function appears generic enough, see if you can move them to Rusty traits.
impl ImageMetadata {
    pub fn from_file(filename: &PathBuf) -> Result<Self, exif::Error> {
        let file = File::open(filename).expect("File doesn't exist");
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        Ok(Self { raw: exif })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, exif::Error> {
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_raw(bytes)?;
        Ok(Self { raw: exif })
    }

    pub fn from_fields<'a>(
        fields: impl Iterator<Item = &'a exif::Field>,
    ) -> Result<Self, exif::Error> {
        let mut writer = Writer::new();
        let mut buf = std::io::Cursor::new(Vec::new());
        let mut count = 0;
        for f in fields {
            writer.push_field(&f);
            count += 1;
        }
        writer.write(&mut buf, false).expect("Unable to write");
        let exif = ImageMetadata::from_bytes(buf.into_inner())?;
        Ok(exif)
    }

    //Possible code duplication, refactor
    pub fn to_bytes(self) -> Bytes {
        let mut writer = Writer::new();
        let mut buf = std::io::Cursor::new(Vec::new());
        for f in self.raw.fields() {
            writer.push_field(&f);
        }
        writer.write(&mut buf, false).expect("Unable to write");
        Bytes::copy_from_slice(&buf.into_inner())
    }

    pub fn print(self, show: bool) {
        for f in self.raw.fields() {
            if is_personally_identifiable(f) {
                if show {
                    println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
                } else {
                    println!("{} {} ******", f.tag, f.ifd_num);
                }
            } else {
                println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
            }
        }
    }
}

#[derive(Debug)]
pub enum ImageMetadataViews {
    All,
    PersonallyIdentifiable,
    TimeStamp,
    Geo,
    Device,
}

fn is_personally_identifiable(field: &exif::Field) -> bool {
    is_geo(field) || is_device(field)
}

fn is_geo(field: &exif::Field) -> bool {
    field.tag.to_string().contains("GPS")
}

fn is_device(field: &exif::Field) -> bool {
    match field.tag.to_string().as_ref() {
        "Software" | "Make" | "Model" | "LensMake" | "LensModel" => true,
        _ => false,
    }
}

pub fn filter_metadata<'a>(
    image: &'a Image,
    view: ImageMetadataViews,
    flip: bool,
) -> impl Iterator<Item = &'a exif::Field> {
    return image.exif.raw.fields().filter(move |&x| {
        let result = match view {
            ImageMetadataViews::All => true,
            ImageMetadataViews::PersonallyIdentifiable => is_personally_identifiable(x),
            ImageMetadataViews::TimeStamp => x.tag.to_string().contains("DateTime"),
            ImageMetadataViews::Geo => is_geo(x),
            ImageMetadataViews::Device => is_device(x),
        };
        if flip {
            !result
        } else {
            result
        }
    });
}

pub fn write_image(outfile: &PathBuf, image: Image) {
    let bytes = image.exif.to_bytes();
    let out = File::create(outfile).expect("Unable to create file");
    let mut jpeg = image.raw;
    jpeg.set_exif(Some(bytes));
    jpeg.encoder()
        .write_to(out)
        .expect("Unable to write to file");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn tests_load_read() {
        let image_path = PathBuf::from("test_images/meme.jpeg");
        let image = Image::from_file(&image_path);
        image.exif.print(true);
        println!("{:?}", image.raw)
    }

    #[test]
    #[ignore]
    fn tests_to_bytes() {
        let image_path = PathBuf::from("test_images/meme.jpeg");
        let image = Image::from_file(&image_path);
        let bytes = image.exif.to_bytes();
        println!("{:?}", bytes);
    }

    #[test]
    #[ignore]
    fn tests_write_image() {
        let image = Image::from_file(&PathBuf::from("test_images/meme.jpeg"));
        write_image(&PathBuf::from("out.jpeg"), image)
    }

    #[test]
    #[ignore]
    fn test_filter_metadata() {
        let image = Image::from_file(&PathBuf::from("test_images/meme.jpeg"));
        let filtered = filter_metadata(&image, ImageMetadataViews::PersonallyIdentifiable, false);
        for f in filtered {
            println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
        }
    }
}
