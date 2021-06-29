use exif;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opts {
    input_file: PathBuf,
    output_file: PathBuf,
}

fn read_exif(path: PathBuf) {
    let file = std::fs::File::open(path).expect("File doesn't exist");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .expect("Can't read from container");
    for f in exif.fields() {
        if !f.tag.to_string().contains("GPS") {
            println!(
                "{} {} {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }
}

fn main() {
    let args = Opts::from_args();
    println!("{:?}", args);
    read_exif(args.input_file);
}
