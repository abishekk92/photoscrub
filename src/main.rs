#![allow(unused_variables)]
use exif;
use exif::experimental::Writer;
use img_parts::jpeg::Jpeg;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO
// * Figure out how to write the exif data to a new file.
// * Integrate with a faker and overwrite.
// * Resolve filter vs select. list wants to filter out and scrub and overwrite wants to use the values remaining after filter.
// * Figure out a way to organize the code better, there is a lot of shared data passed around.
// * Figure out how to autogenerate documentation.

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(parse(from_os_str), short)]
    input_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Option<Command>,
    #[structopt(short, long, possible_values = &Filter::variants(), case_insensitive = true)]
    filter: Filter,
}

#[derive(StructOpt, Debug)]
enum Command {
    List {
        #[structopt(short, long)]
        show: bool,
    },
    Scrub {
        #[structopt(parse(from_os_str), short)]
        output_file: Option<PathBuf>,
    },
    Overwrite {
        #[structopt(parse(from_os_str), short)]
        output_file: Option<PathBuf>,
    },
}

arg_enum! {
    #[derive(StructOpt, Debug, PartialEq)]
    enum Filter {
        All,
        Device,
        Geo,
    }
}

fn read_exif(path: PathBuf) -> Result<exif::Exif, exif::Error> {
    let file = std::fs::File::open(path).expect("File doesn't exist");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    return exifreader.read_from_container(&mut bufreader);
}

fn filter_fields<'a>(
    exif_data: &'a exif::Exif,
    filter: &'a crate::Filter,
) -> impl Iterator<Item = &'a exif::Field> {
    return exif_data.fields().filter(move |&x| match filter {
        crate::Filter::All => true,
        crate::Filter::Geo => x.tag.to_string().contains("GPS"),
        crate::Filter::Device => match x.tag.to_string().as_ref() {
            "Software" | "Make" | "Model" | "LensMake" | "LensModel" => true,
            _ => false,
        },
    });
}

fn print_metdata<'a>(fields: impl Iterator<Item = &'a exif::Field>, show: bool) {
    for f in fields {
        if show {
            println!("{} {} {}", f.tag, f.ifd_num, f.display_value().with_unit(f));
        } else {
            println!("{} {} ******", f.tag, f.ifd_num);
        }
    }
}

fn scrub<'a>(fields: impl Iterator<Item = &'a exif::Field>, output_file: PathBuf) {
    let mut writer = Writer::new();
    let mut buf = std::io::Cursor::new(Vec::new());
    let mut count: i8 = 0;
    for f in fields {
        count += 1;
    }
    if count > 0 {
        writer.write(&mut buf, false).expect("asdfasfsa");
    }
    let mut output = File::create(output_file).expect("Can't create file");
    output.write_all(&buf.into_inner()).expect("Failed writing")
}

fn overwrite<'a>(fields: impl Iterator<Item = &'a exif::Field>, output_file: PathBuf) {
    for f in fields {
        println!("Overwriting {} {} ******", f.tag, f.ifd_num);
    }
}

fn main() {
    let args = Opts::from_args();
    let exif_data = read_exif(args.input_file).expect("File not found");
    let filtered = filter_fields(&exif_data, &args.filter);

    match args.cmd {
        Some(Command::List { show }) => print_metdata(filtered, show),
        Some(Command::Scrub { output_file }) => scrub(filtered, output_file.unwrap()),
        Some(Command::Overwrite { output_file }) => overwrite(filtered, output_file.unwrap()),
        _ => println!("Not supported yet"),
    }
}
