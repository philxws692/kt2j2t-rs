use clap::Parser;
use kt2j2t_rs::kt2j2t;

#[derive(Parser, Debug)]
#[command(author, version, about, name = "kt2j2")]
pub struct Args {
    #[arg(short, long)]
    /// File with JSON to transform
    input: String,

    #[arg(short, long)]
    /// File where transformed JSON should be outputted
    output: String,
}

fn main() {
    let args = Args::parse();

    let input_file = std::fs::read_to_string(args.input).unwrap();

    let output_file_arg = args.output;

    match kt2j2t(&input_file) {
        Ok(transformed_json) => {
            println!("JSON was successfully transformed");
            std::fs::write(
                output_file_arg,
                serde_json::to_string_pretty(&transformed_json).unwrap(),
            )
            .unwrap();
        }
        Err(e) => {
            eprintln!("error with transforming json: {}", e);
        }
    }
}
