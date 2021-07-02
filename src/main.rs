#![allow(unused_variables)]
use exif;
use exif::experimental::Writer;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

//TODO Figure out autogenerate of documentation

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(parse(from_os_str), short)]
    input_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Option<Command>,
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
        #[structopt(parse(from_os_str), short)]
        output_file: Option<PathBuf>,
    },
    Overwrite {
        #[structopt(possible_values = &Filter::variants(), case_insensitive = true)]
        filter: Filter,
        #[structopt(parse(from_os_str), short)]
        output_file: Option<PathBuf>,
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

fn main() {
    let args = Opts::from_args();
    let exif = read_exif(args.input_file).expect("File not found");

    match args.cmd {
        Some(Command::List { filter, show }) => {
            println!("Metadata");
            let fields = exif.fields().filter(|x| match filter {
                crate::Filter::All => true,
                crate::Filter::Geo => x.tag.to_string().contains("GPS"),
                crate::Filter::Device => {
                    x.tag.to_string() == "Make" || x.tag.to_string() == "Model"
                }
            });
            for f in fields {
                if show {
                    println!(
                        "{} {} {}",
                        f.tag,
                        f.ifd_num,
                        f.display_value().with_unit(())
                    );
                } else {
                    println!("{} {} ******", f.tag, f.ifd_num);
                }
            }
        }
        // Lack of support for keyword arguments make this function call look ugly
        // TODO Figure out a better way to do this.
        Some(Command::Scrub {
            filter,
            output_file,
        }) => {
            println!("Metadata");
            let fields = exif.fields().filter(|x| match filter {
                crate::Filter::All => !true,
                crate::Filter::Geo => !x.tag.to_string().contains("GPS"),
                crate::Filter::Device => {
                    !(x.tag.to_string() == "Make" || x.tag.to_string() == "Model")
                }
            });
            let mut writer = Writer::new();
            let mut buf = std::io::Cursor::new(Vec::new());
            let mut count: i8 = 0;
            for f in fields {
                println!("Scrubbing {} {} ******", f.tag, f.ifd_num);
                writer.push_field(&f);
                count += 1;
            }
            if count > 0 {
                writer.write(&mut buf, false).expect("asdfasfsa");
                println!("{:?}", buf.into_inner())
            }
        }
        Some(Command::Overwrite {
            filter,
            output_file,
        }) => {
            println!("Metadata");
            let fields = exif.fields().filter(|x| match filter {
                crate::Filter::All => true,
                crate::Filter::Geo => x.tag.to_string().contains("GPS"),
                crate::Filter::Device => {
                    x.tag.to_string() == "Make" || x.tag.to_string() == "Model"
                }
            });
            for f in fields {
                println!("Overwriting {} {} ******", f.tag, f.ifd_num);
            }
        }
        _ => println!("Not supported yet"),
    }
}
