use std::env;

struct Opts {
    input_file: String,
    output_file: String,
}

impl Opts {
    fn from_args(args: &Vec<String>) -> Opts {
        let input_file = &args[1];
        let output_file = &args[2];
        return Opts {
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts: Opts = Opts::from_args(&args);
    println!("{} {}", opts.input_file, opts.output_file)
}
