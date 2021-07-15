#![allow(unused_variables)]
mod image;

use crate::image::{write_image, Image, ImageMetadata};
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO
// * [X] Figure out how to write the jpeg + exif data to a new file.
// * [X] Figure out a way to organize the code better, there is a lot of shared data passed around.
// * [ ] Resolve filter vs select. list wants to filter out and scrub and overwrite wants to use the values remaining after filter.
// * [ ] Integrate with a faker and overwrite.
// * [ ] Figure out how to autogenerate documentation.

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(parse(from_os_str), short)]
    input_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
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
        output_file: PathBuf,
    },
    Overwrite {
        #[structopt(parse(from_os_str), short)]
        output_file: PathBuf,
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
    let mut image = Image::from_file(&args.input_file);
    let filtered = filter_fields(&image.exif.raw, &args.filter);

    match args.cmd {
        Command::List { show } => {
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            image.exif.print(show);
        }
        Command::Scrub { output_file } => {
            //Scrub
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            write_image(&output_file, image);
        }
        Command::Overwrite { output_file } => {
            //Overwrite
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            write_image(&output_file, image);
        }
    }
}
