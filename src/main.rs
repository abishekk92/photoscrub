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
    #[derive(StructOpt, Debug)]
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

//TODO Find out a better way to group the parameters, filter and show belog together.
fn print_metadata(exif: exif::Exif, filter: crate::Filter, show: bool) {
    // Filter by options. all, device, geo
    println!("Displaying {} fields", filter);
    println!("\n========\n");
    for f in exif.fields() {
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
    println!("\n========\n");
}

// fn scrub(exif: exif::Exif) -> exif::Exif {}

// fn overwrite(exif: exif::Exif) -> exif::Exif {}
fn main() {
    let args = Opts::from_args();
    let exif = read_exif(args.input_file).expect("File not found");
    match args.cmd {
        Some(Command::List { filter, show }) => {
            print_metadata(exif, filter, show);
        }
        _ => println!("Not supported yet"),
    }
}
