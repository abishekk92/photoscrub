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

trait ExifData {
    fn filter_by(self, filter: crate::Filter) -> Self;
    fn print_metadata(self, show: bool);
    fn scrub(self) -> Self;
    fn overwrite(self) -> Self;
}

impl ExifData for exif::Exif {
    fn filter_by(self, filter: crate::Filter) -> Self {
        return self;
    }
    fn print_metadata(self, show: bool) {
        // println!("Displaying {} fields", filter);
        // let fields = self.fields().filter(|x| match filter {
        //     crate::Filter::All => true,
        //     crate::Filter::Geo => x.tag.to_string().contains("GPS"),
        //     crate::Filter::Device => x.tag.to_string() == "Make" || x.tag.to_string() == "Model",
        // });
        println!("========");
        for f in self.fields() {
            if show {
                println!(
                    "{} {} {}",
                    f.tag,
                    f.ifd_num,
                    f.display_value().with_unit(&self)
                );
            } else {
                println!("{} {} ******", f.tag, f.ifd_num);
            }
        }
        println!("========");
    }
    fn scrub(self) -> Self {
        println!("Scrubbed fields");
        return self;
    }
    fn overwrite(self) -> Self {
        println!("Overwrite fields");
        return self;
    }
}

fn main() {
    let args = Opts::from_args();
    let exif = read_exif(args.input_file).expect("File not found");

    match args.cmd {
        Some(Command::List { filter, show }) => exif.filter_by(filter).print_metadata(show),
        // Lack of support for keyword arguments make this function call look ugly
        // TODO Figure out a better way to do this.
        Some(Command::Scrub { filter }) => exif.filter_by(filter).scrub().print_metadata(false),
        Some(Command::Overwrite { filter }) => {
            exif.filter_by(filter).overwrite().print_metadata(false)
        }
        _ => println!("Not supported yet"),
    }
}
