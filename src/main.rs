#![allow(unused_variables)]
mod exif_utils;
mod image;

use crate::image::Image;
use exif;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO
// * [ ] Figure out how to write the jpeg + exif data to a new file.
// * [ ] Integrate with a faker and overwrite.
// * [ ] Resolve filter vs select. list wants to filter out and scrub and overwrite wants to use the values remaining after filter.
// * [ ] Figure out a way to organize the code better, there is a lot of shared data passed around.
// * [ ] Figure out how to autogenerate documentation.

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

fn main() {
    let args = Opts::from_args();
    // let exif_data = exif_utils::read_exif(&args.input_file).expect("File not found");
    let image = Image::from_file(&args.input_file);
    let filtered = filter_fields(&image.metadata.raw, &args.filter);

    match args.cmd {
        Some(Command::List { show }) => exif_utils::print_metdata(filtered, show),
        Some(Command::Scrub { output_file }) => exif_utils::scrub(filtered, output_file.unwrap()),
        Some(Command::Overwrite { output_file }) => {
            exif_utils::overwrite(filtered, output_file.unwrap())
        }
        _ => println!("Not supported yet"),
    }
}
