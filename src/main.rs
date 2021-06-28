use exif;
use std::env;

struct Opts {
    input_file: String,
    // output_file: String,
}

impl Opts {
    fn from_args(args: &Vec<String>) -> Result<Opts, &str> {
        if args.len() < 2 {
            return Err("Commandline arguments missing, dunno what to do!");
        }
        let input_file = &args[1];
        // let output_file = &args[2];
        return Ok(Opts {
            input_file: input_file.to_string(),
            // output_file: output_file.to_string(),
        });
    }
}

fn read_exif(path: String) {
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
    let args: Vec<String> = env::args().collect();
    match Opts::from_args(&args) {
        Ok(opts) => read_exif(opts.input_file),
        Err(error_str) => println!("Error:{}", error_str),
    }
}
