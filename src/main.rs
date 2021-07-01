#![allow(unused_variables)]
use exif;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO Figure out autogenerate of documentation

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(parse(from_os_str), short)]
    input_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Option<Command>,
    #[structopt(parse(from_os_str), short)]
    output_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
// Ideally filter should be more generalizable
// TODO Find a way to avoid repeating filter
enum Command {
    List {
        #[structopt(possible_values = &Filter::variants(), case_insensitive = true)]
        filter: Filter,
        #[structopt(short, long)]
        show: bool,
    },
    Scrub {
        #[structopt(possible_values = &Filter::variants(), case_insensitive = true)]
        filter: Filter,
    },
    Overwrite {
        #[structopt(possible_values = &Filter::variants(), case_insensitive = true)]
        filter: Filter,
    },
}

//TODO Document which fields constitute as device ids vs geo ids.

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

//TODO Find out a better way to group the parameters, filter and show belong together.
fn print_metadata(exif: exif::Exif, filter: crate::Filter, show: bool) {
    println!("\n========");
    println!("Displaying {} fields", filter);
    println!("========\n");
    let fields = exif.fields().filter(|x| match filter {
        crate::Filter::All => true,
        crate::Filter::Geo => x.tag.to_string().contains("GPS"),
        crate::Filter::Device => x.tag.to_string() == "Make" || x.tag.to_string() == "Model",
    });
    for f in fields {
        if show {
            println!(
                "{} {} {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        } else {
            println!("{} {} ******", f.tag, f.ifd_num);
        }
    }
}

fn scrub(exif: exif::Exif, filter: crate::Filter) {
    println!("Scrubbed {} fields", filter)
}

fn overwrite(exif: exif::Exif, filter: crate::Filter) {
    println!("Overwriting {} successful", filter)
}

fn main() {
    let args = Opts::from_args();
    let exif = read_exif(args.input_file).expect("File not found");
    match args.cmd {
        Some(Command::List { filter, show }) => {
            print_metadata(exif, filter, show);
        }
        Some(Command::Scrub { filter }) => scrub(exif, filter),
        Some(Command::Overwrite { filter }) => overwrite(exif, filter),
        _ => println!("Not supported yet"),
    }
}
