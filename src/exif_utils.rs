use exif;
use exif::experimental::Writer;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn read_exif(path: &PathBuf) -> Result<exif::Exif, exif::Error> {
    let file = File::open(path).expect("File doesn't exist");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    return exifreader.read_from_container(&mut bufreader);
}

pub fn print_metdata<'a>(fields: impl Iterator<Item = &'a exif::Field>, show: bool) {
    for f in fields {
        if show {
            println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
        } else {
            println!("{} {} ******", f.tag, f.ifd_num);
        }
    }
}

pub fn overwrite<'a>(fields: impl Iterator<Item = &'a exif::Field>, output_file: PathBuf) {
    for f in fields {
        println!("Overwriting {} {} ******", f.tag, f.ifd_num);
    }
}

pub fn scrub<'a>(fields: impl Iterator<Item = &'a exif::Field>, output_file: PathBuf) {
    let mut writer = Writer::new();
    let mut buf = std::io::Cursor::new(Vec::new());
    let mut count: i8 = 0;
    for f in fields {
        count += 1;
        writer.push_field(f)
    }
    if count > 0 {
        writer.write(&mut buf, false).expect("asdfasfsa");
    }
    // set thumbnail
    let mut output = File::create(output_file).expect("Can't create file");
    output.write_all(&buf.into_inner()).expect("Failed writing")
}
