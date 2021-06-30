use exif;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

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

arg_enum! {
    #[derive(StructOpt, Debug)]
    enum Filter {
        All,
        Device,
        Geo,
    }
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
