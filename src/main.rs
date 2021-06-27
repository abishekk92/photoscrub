use std::env;

struct Opts {
    input_file: String,
    output_file: String,
}

impl Opts {
    fn from_args(args: &Vec<String>) -> Result<Opts, &str> {
        if args.len() < 3 {
            return Err("Commandline arguments missing, dunno what to do!");
        }
        let input_file = &args[1];
        let output_file = &args[2];
        return Ok(Opts {
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        });
    }
}

fn read_file(input_file: String) {
    println!("Reading {}", input_file)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match Opts::from_args(&args) {
        Ok(opts) => read_file(opts.input_file),
        Err(error_str) => println!("Error:{}", error_str),
    }
}
