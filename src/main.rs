use exif;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
enum Filter {
    All,
    Device,
    Geo,
}

impl FromStr for Filter {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_ref() {
            "all" => Ok(Filter::All),
            "device" => Ok(Filter::Device),
            "geo" => Ok(Filter::Geo),
            // Returns all by default
            _ => Ok(Filter::All),
        }
    }
}

impl Filter {
    fn variants() -> [&'static str; 3] {
        return ["all", "device", "geo"];
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
