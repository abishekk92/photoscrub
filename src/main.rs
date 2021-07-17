#![allow(unused_variables)]
mod image;

use crate::image::{write_image, Image, ImageMetadata};
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO
// * [X] Figure out how to write the jpeg + exif data to a new file.
// * [X] Figure out a way to organize the code better, there is a lot of shared data passed around.
// * [X] Resolve filter vs select. list wants to filter out and scrub and overwrite wants to use the values remaining after filter.
// * [ ] Integrate with a faker and overwrite.
// * [ ] Figure out how to autogenerate documentation.

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(parse(from_os_str), short)]
    input_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    List {
        #[structopt(short, long)]
        show: bool,
        #[structopt(short, long, possible_values = &Filter::variants(), case_insensitive = true, default_value="personal")]
        filter: Filter,
    },
    Scrub {
        #[structopt(parse(from_os_str), short)]
        output_file: PathBuf,
        #[structopt(short, long, possible_values = &Filter::variants(), case_insensitive = true, default_value="personal")]
        filter: Filter,
    },
    Overwrite {
        #[structopt(parse(from_os_str), short)]
        output_file: PathBuf,
        #[structopt(short, long, possible_values = &Filter::variants(), case_insensitive = true, default_value="personal")]
        filter: Filter,
    },
}

arg_enum! {
    #[derive(StructOpt, Debug, PartialEq)]
    enum Filter {
        All,
        Personal,
        Geo,
        Device,
        Time,
    }
}

fn filter_fields(
    image: &image::Image,
    filter: Filter,
    flip: bool,
) -> impl Iterator<Item = &exif::Field> {
    return match filter {
        crate::Filter::All => image::filter_metadata(&image, image::ImageMetadataViews::All, flip),
        crate::Filter::Time => {
            image::filter_metadata(&image, image::ImageMetadataViews::TimeStamp, flip)
        }
        crate::Filter::Personal => image::filter_metadata(
            &image,
            image::ImageMetadataViews::PersonallyIdentifiable,
            flip,
        ),
        crate::Filter::Geo => image::filter_metadata(&image, image::ImageMetadataViews::Geo, flip),
        crate::Filter::Device => {
            image::filter_metadata(&image, image::ImageMetadataViews::Device, flip)
        }
    };
}

fn main() {
    let args = Opts::from_args();
    let mut image = Image::from_file(&args.input_file);

    match args.cmd {
        Command::List { show, filter } => {
            //Filter everything as mentioned here
            //p -i i.jpeg list -f all|personal -s
            let filtered = filter_fields(&image, filter, false);
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            image.exif.print(show);
        }
        Command::Scrub {
            output_file,
            filter,
        } => {
            //Scrub
            //Filter everything but
            //p -i i.jpeg list -f all|personal -s
            let filtered = filter_fields(&image, filter, true);
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            write_image(&output_file, image);
        }
        Command::Overwrite {
            output_file,
            filter,
        } => {
            //Split the iterator into two, overwrite one and the merge the remaining.
            let filtered = filter_fields(&image, filter, false);
            //Overwrite
            image.exif = ImageMetadata::from_fields(filtered).unwrap();
            write_image(&output_file, image);
        }
    }
}
